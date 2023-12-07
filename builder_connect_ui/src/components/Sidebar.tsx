import { useEffect, useState } from "react";
import { EnvelopeIcon } from "@heroicons/react/24/solid";
import { view_profile } from "@/libs/functions";
import useWebSocket from "react-use-websocket";
import SidebarComponent from "./SidebarComponent";

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

  //TODO: How to get last message from websocket for all rooms?
  // possible separate into sidebar component and pass in match_room_id as prop
  // const { lastJsonMessage } = useWebSocket(
  //   `ws://${process.env.NEXT_PUBLIC_BASE_URL?.slice(7)}/chat/${match_room_id}`
  // );

  // console.log("lastJsonMessage", lastJsonMessage);

  function is_online(last_seen_date: Date): boolean {
    const now = new Date();
    const diff = now.getTime() - last_seen_date.getTime();
    const diff_minutes = Math.round(diff / 60000);
    return diff_minutes < 10;
  }

  // function get_last_message(room_id: string): string | any {
  //   if (
  //     room_id === match_room_id &&
  //     lastJsonMessage &&
  //     lastJsonMessage.chat_type === "TEXT"
  //   )
  //     return lastJsonMessage;
  //   if (all_messages && all_messages.has(room_id)) {
  //     const room_messages = all_messages.get(room_id) ?? [];
  //     const last_message =
  //       room_messages.length > 0
  //         ? room_messages[room_messages.length - 1]
  //         : null;
  //     console.log("last_message: ", last_message);
  //     return last_message;
  //   }
  //   return "Loading...";
  // }

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

  // useEffect(() => {
  //   rooms.sort((a: any[], b: any[]) => {
  //     const a_last_message = get_last_message(a[1]); // a[1] = a_room_id
  //     const b_last_message = get_last_message(b[1]);
  //     if (a_last_message && b_last_message) {
  //       return (
  //         new Date(b_last_message.created_at).getTime() -
  //         new Date(a_last_message.created_at).getTime()
  //       );
  //     }
  //     return 0;
  //   });
  // }, [lastJsonMessage, rooms]);

  return (
    <div>
      {rooms && rooms.length > 0 ? (
        rooms.map((room: any, index: number) => (
          <SidebarComponent
            key={index}
            room={room}
            last_message={all_messages.get(room[1])?.slice(-1)[0]}
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
