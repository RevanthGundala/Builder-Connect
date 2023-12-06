import { useEffect, useState } from "react";
import { EnvelopeIcon } from "@heroicons/react/24/solid";
import { view_profile } from "@/libs/functions";
import useWebSocket from "react-use-websocket";

export default function Sidebar({
  sub_id,
  profile,
  all_messages,
  match_room_id,
  set_profile,
  set_match_profile,
  set_match_room_id,
}: {
  sub_id: string;
  profile: any;
  all_messages: Map<string, any[]>;
  match_room_id: string;
  set_profile: React.Dispatch<React.SetStateAction<any>>;
  set_match_profile: React.Dispatch<React.SetStateAction<any>>;
  set_match_room_id: React.Dispatch<React.SetStateAction<any>>;
}) {
  const [match_sub_id, set_match_sub_id] = useState("");
  const [rooms, set_rooms] = useState<any[]>([]);
  const [image_error, set_image_error] = useState(false);

  const { lastJsonMessage } = useWebSocket(
    `ws://${process.env.NEXT_PUBLIC_BASE_URL?.slice(7)}/chat/${match_room_id}`
  );

  console.log("lastJsonMessage", lastJsonMessage);

  function get_last_message(room_id: string): string | any {
    if (
      room_id === match_room_id &&
      lastJsonMessage &&
      lastJsonMessage.chat_type === "TEXT"
    )
      return lastJsonMessage;
    if (all_messages && all_messages.has(room_id)) {
      const room_messages = all_messages.get(room_id) ?? [];
      const last_message =
        room_messages.length > 0
          ? room_messages[room_messages.length - 1]
          : null;
      return last_message;
    }
    return "Loading...";
  }

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
              <div className="flex flex-row bg-gray-300 py-4 px-2">
                <div className="flex flex-row flex-1 px-2 space-x-3">
                  <img
                    src={
                      image_error
                        ? "/images/default_user.png"
                        : room[0]?.image_url
                    }
                    onError={() => set_image_error(true)}
                    alt={room[0].username}
                    className="w-6 h-6 object-cover rounded-full"
                  />
                  <div className="flex flex-col">
                    <p>{room[0].username}</p>
                    <p>{get_last_message(room[1])?.content ?? "New match!"}</p>
                  </div>
                </div>
                <div className="flex flex-row px-2">
                  {get_last_message(room[1])?.created_at
                    ? new Date(
                        get_last_message(room[1])?.created_at
                      ).toLocaleString(undefined, {
                        hour: "numeric",
                        minute: "numeric",
                      })
                    : ""}
                </div>
              </div>
            ) : (
              <div className="flex flex-row bg-gray-300 opacity-70 py-4 px-2">
                <div className="flex flex-row flex-1 px-2 space-x-3">
                  <img
                    src={
                      image_error
                        ? "/images/default_user.png"
                        : room[0]?.image_url
                    }
                    onError={() => set_image_error(true)}
                    alt={room[0].username}
                    className="w-6 h-6 object-cover rounded-full"
                  />
                  <div className="flex flex-col">
                    <p>{room[0].username}</p>
                    <p>{get_last_message(room[1])?.content ?? "New match!"}</p>
                  </div>
                </div>
                <div className="flex flex-row px-2">
                  {get_last_message(room[1])?.created_at
                    ? new Date(
                        get_last_message(room[1])?.created_at
                      ).toLocaleString(undefined, {
                        hour: "numeric",
                        minute: "numeric",
                      })
                    : ""}
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
