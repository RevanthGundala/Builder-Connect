import React, { useEffect, useState } from "react";
import useWebSocket from "react-use-websocket";
export default function SidebarComponent({
  room,
  match_sub_id,
  room_to_last_message,
  set_match_sub_id,
  set_match_room_id,
  set_room_to_last_message,
}: {
  room: any;
  match_sub_id: string;
  room_to_last_message: Map<string, any>;
  set_match_sub_id: React.Dispatch<React.SetStateAction<string>>;
  set_match_room_id: React.Dispatch<React.SetStateAction<string>>;
  set_room_to_last_message: React.Dispatch<React.SetStateAction<any>>;
}) {
  const [image_error, set_image_error] = useState(false);
  const [is_online, set_is_online] = useState(false);

  const { lastJsonMessage } = useWebSocket(
    `ws://${process.env.NEXT_PUBLIC_BASE_URL?.slice(7)}/chat/${room[1]}`
  );

  function check_is_online(last_seen: string | undefined) {
    if (!last_seen || last_seen === "") return;
    try {
      const now = new Date();
      const last_seen_date = new Date(last_seen);
      const diff = now.getTime() - last_seen_date.getTime();
      const diff_minutes = Math.round(diff / 60000);
      diff_minutes < 5 ? set_is_online(true) : set_is_online(false);
    } catch (e) {
      console.log(e);
      set_is_online(false);
    }
  }

  useEffect(() => {
    if (
      lastJsonMessage &&
      lastJsonMessage.chat_type === "TEXT" &&
      JSON.stringify(lastJsonMessage) !==
        JSON.stringify(room_to_last_message.get(room[1]))
    ) {
      set_room_to_last_message(
        (prev: Map<string, any>) => new Map(prev.set(room[1], lastJsonMessage))
      );
    }
  }, [lastJsonMessage]);

  useEffect(() => {
    room && room[0] && room[0].image_url
      ? set_image_error(false)
      : set_image_error(true);
  }, [room, room[0]?.image_url]);

  useEffect(() => {
    check_is_online(room[0].last_seen);
    const interval = setInterval(() => {
      check_is_online(room[0].last_seen);
    }, 10000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div
      className="flex flex-col text-black"
      onClick={() => {
        set_match_sub_id(room[0].sub_id);
        set_match_room_id(room[1]);
      }}
    >
      <div
        className={
          room[0].sub_id === match_sub_id
            ? "flex flex-row bg-gray-300 py-4 px-2"
            : "flex flex-row bg-gray-300 opacity-70 py-4 px-2"
        }
      >
        <div className="flex flex-row flex-1 px-2 space-x-3">
          <img
            src={image_error ? "/images/default_user.png" : room[0]?.image_url}
            onError={() => set_image_error(true)}
            alt={room[0].username}
            className="w-6 h-6 object-cover rounded-full"
          />
          <div className="flex flex-col">
            <p>{room[0].username}</p>
            <p>{room_to_last_message.get(room[1])?.content ?? "New Match!"}</p>
          </div>
          <span
            className={
              is_online
                ? "w-2 h-2 bg-green-500 rounded-full absolute"
                : "w-2 h-2 bg-red-500 rounded-full absolute"
            }
          ></span>
        </div>
        <div className="flex flex-row px-2">
          {room_to_last_message.get(room[1])?.created_at
            ? new Date(
                room_to_last_message.get(room[1])?.created_at
              ).toLocaleString(undefined, {
                hour: "numeric",
                minute: "numeric",
              })
            : ""}
        </div>
      </div>
    </div>
  );
}
