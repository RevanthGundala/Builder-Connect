import { useEffect, useState, useMemo } from "react";
import { EnvelopeIcon } from "@heroicons/react/24/solid";
import { view_profile } from "@/libs/functions";
import SidebarComponent from "./SidebarComponent";

export default function Sidebar({
  sub_id,
  profile,
  all_messages,
  room_to_last_message,
  set_profile,
  set_match_profile,
  set_match_room_id,
  set_room_to_last_message,
}: {
  sub_id: string;
  profile: any;
  all_messages: Map<string, any[]>;
  room_to_last_message: Map<string, any>;
  match_room_id: string;
  set_profile: React.Dispatch<React.SetStateAction<any>>;
  set_match_profile: React.Dispatch<React.SetStateAction<any>>;
  set_match_room_id: React.Dispatch<React.SetStateAction<any>>;
  set_room_to_last_message: React.Dispatch<React.SetStateAction<any>>;
}) {
  const [match_sub_id, set_match_sub_id] = useState("");
  const [rooms, set_rooms] = useState<any[]>([]);

  function is_online(last_seen_date: Date): boolean {
    const now = new Date();
    const diff = now.getTime() - last_seen_date.getTime();
    const diff_minutes = Math.round(diff / 60000);
    return diff_minutes < 10;
  }

  function get_room_order(profile_rooms: any[] = rooms) {
    const old_matches = profile_rooms.filter(
      (room: any[2]) => room_to_last_message.get(room[1]) !== undefined
    );
    old_matches.sort((a: any[2], b: any[2]) => {
      const a_last_message = room_to_last_message.get(a[1]);
      const b_last_message = room_to_last_message.get(b[1]);
      return (
        new Date(b_last_message?.created_at).getTime() -
        new Date(a_last_message?.created_at).getTime()
      );
    });
    set_rooms((prev) => [
      ...old_matches,
      ...prev.filter((room) => !old_matches.includes(room)),
    ]);
  }

  async function get_profile(id: string = sub_id) {
    const profile_data = await view_profile(id, profile);
    id === sub_id ? set_profile(profile_data) : set_match_profile(profile_data);
    if (id === sub_id && profile_data?.matches.length > 0) {
      const profile_rooms = await Promise.all(
        profile_data?.matches.map(async (room: any) => {
          const match_id = room.user_sub_ids[0];
          const url = process.env.NEXT_PUBLIC_BASE_URL + `/view/${match_id}`;
          const res = await fetch(url);
          const match = await res.json();
          return [match, room.room_id];
        })
      );
      set_rooms(profile_rooms);
    }
  }

  useEffect(() => {
    get_profile();
  }, []);

  useEffect(() => {
    get_profile(match_sub_id);
  }, [match_sub_id]);

  useEffect(() => {
    console.log("room_to_last_message: ", room_to_last_message);
    get_room_order();
  }, [room_to_last_message, profile]);

  return (
    <div>
      {rooms && rooms.length > 0 ? (
        rooms.map((room: any, index: number) => (
          <SidebarComponent
            key={index}
            room={room}
            last_message={room_to_last_message.get(room[1])}
            match_sub_id={match_sub_id}
            set_match_sub_id={set_match_sub_id}
            set_match_room_id={set_match_room_id}
          />
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
