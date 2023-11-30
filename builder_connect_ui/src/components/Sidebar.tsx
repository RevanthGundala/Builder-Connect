import { useEffect, useState } from "react";
import { EnvelopeIcon } from "@heroicons/react/24/solid";
import { view_profile } from "@/libs/functions";

export default function Sidebar({
  sub_id,
  profile,
  set_profile,
  set_match_profile,
  set_match_room_id,
}: {
  sub_id: string;
  profile: any;
  set_profile: React.Dispatch<React.SetStateAction<any>>;
  set_match_profile: React.Dispatch<React.SetStateAction<any>>;
  set_match_room_id: React.Dispatch<React.SetStateAction<any>>;
}) {
  const [match_sub_id, set_match_sub_id] = useState("");
  const [rooms, set_rooms] = useState<any[]>([]);

  async function get_profile(id = sub_id) {
    const profile_data = await view_profile(id, profile);
    id === sub_id ? set_profile(profile_data) : set_match_profile(profile_data);
    if (id === sub_id && profile_data?.matches.length > 0) {
      const rooms = await Promise.all(
        profile_data?.matches.map(async (room: any) => {
          const match_id = room.user_sub_ids[0];
          const url = process.env.NEXT_PUBLIC_BASE_URL + `/view/${match_id}`;
          const res = await fetch(url);
          const match = await res.json();
          return [match, room.room_id];
        })
      );
      set_rooms(rooms); // rooms = [match_profile, room_id]
    }
  }

  useEffect(() => {
    get_profile();
    // if (sub_id !== router.query.sub_id) router.push("/Error");
  }, []);

  useEffect(() => {
    get_profile(match_sub_id);
  }, [match_sub_id]);

  return (
    <div>
      {rooms && rooms.length > 0 ? (
        rooms.map((room: any, index: number) => (
          <div
            key={index}
            className="flex flex-col text-black"
            onClick={() => {
              set_match_sub_id(room[0].sub_id);
              set_match_room_id(room[1]);
            }}
          >
            {room[0].sub_id === match_sub_id ? (
              <div className="flex flex-row space-x-2 bg-gray-300 py-4 px-2">
                <img
                  src={room[0].image_url}
                  alt={room[0].username}
                  className="w-6 h-6 object-cover rounded-full"
                />
                <div className="flex flex-col">
                  <p>{room[0].username}</p>
                  <p>Placeholder for message</p>
                </div>
              </div>
            ) : (
              <div className="flex flex-row space-x-2 bg-gray-300 opacity-70 py-4 px-2">
                <img
                  src={room[0].image_url}
                  alt={room[0].username}
                  className="w-6 h-6 object-cover rounded-full"
                />
                <div className="flex flex-col">
                  <p>{room[0].username}</p>
                  <p>Placeholder for message</p>
                </div>
              </div>
            )}
          </div>
        ))
      ) : (
        <div className="pt-4 bg-cover bg-center">
          <div className="flex flex-col space-y-2">
            <EnvelopeIcon className="w-16 h-16 text-white mx-auto" />
            <p className="text-white text-xl text-center">No matches yet!</p>
          </div>
        </div>
      )}
    </div>
  );
}
