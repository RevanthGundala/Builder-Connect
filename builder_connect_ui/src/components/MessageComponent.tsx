import React, { useEffect, useState, useRef } from "react";
import { PhotoIcon, ArrowRightIcon } from "@heroicons/react/24/solid";
import Conversation from "@/components/Conversation";
import useWebSocket from "react-use-websocket";

export default function MessageComponent({
  match_profile,
  sub_id,
  room_id,
  all_messages,
}: {
  match_profile: any;
  sub_id: string | undefined;
  room_id: string;
  all_messages: Map<string, any[]>;
}) {
  const [text, set_text] = useState("");
  const [is_typing, set_is_typing] = useState(false);
  const [image_error, set_image_error] = useState(false);
  const [socket_messages, set_socket_messages] = useState<any[]>();
  const ref = useRef<HTMLDivElement | null>(null);

  const { lastJsonMessage, sendJsonMessage } = useWebSocket(
    `ws://${process.env.NEXT_PUBLIC_BASE_URL?.slice(7)}/chat/${room_id}`
  );

  function handle_typing(mode: string) {
    mode === "IN" ? set_is_typing(true) : set_is_typing(false);
  }

  function submit_message(e: any) {
    e.preventDefault();
    if (!room_id) {
      alert("Please select room");
    } else {
      const data = {
        room_id: room_id,
        user_sub_id: sub_id,
        chat_type: "TEXT",
        content: text,
      };
      sendJsonMessage(data);
      set_text("");
    }
  }

  useEffect(() => {
    match_profile && match_profile.image_url
      ? set_image_error(false)
      : set_image_error(true);
  }, [match_profile, match_profile?.image_url]);

  useEffect(() => {
    if (ref.current) {
      ref.current.scrollTop = ref.current.scrollHeight;
    }
  }, [socket_messages]);

  useEffect(() => {
    if (all_messages.get(room_id)) {
      set_socket_messages(all_messages.get(room_id));
    }
  }, [room_id, all_messages]);

  useEffect(() => {
    if (lastJsonMessage && lastJsonMessage.chat_type === "TEXT") {
      set_socket_messages((prev: any[] | undefined) => [
        ...(prev ?? []),
        lastJsonMessage,
      ]);
    }
  }, [lastJsonMessage]);

  return (
    <>
      <div className="flex flex-col bg-gray-200 w-5/6 mx-auto my-8">
        <div className="flex flex-col flex-1">
          <header className="bg-gray-300 border-b border-gray-400 py-6 px-2">
            <div className="flex flex-col space-y-1 items-center">
              <img
                src={
                  image_error
                    ? "/images/default_user.png"
                    : match_profile?.image_url
                }
                onError={() => set_image_error(true)}
                alt={match_profile?.username}
                className="w-12 h-12 object-cover rounded-full"
              />
              <div className="text-md text-black">
                <p>{match_profile?.username}</p>
              </div>
            </div>
          </header>
          <main
            ref={ref}
            className="flex flex-col border-b border-gray-400 max-h-[600px] min-h-[600px] overflow-auto"
          >
            <Conversation
              messages={socket_messages ?? []}
              sub_id={sub_id ?? ""}
            />
          </main>
        </div>
        <footer className="flex flex-col">
          <form
            className="flex flex-row items-center space-x-2 p-4"
            onSubmit={submit_message}
          >
            <PhotoIcon className="w-10 h-10 text-black" />
            <input
              type="text"
              placeholder="Type a message..."
              className="flex flex-1 h-15 border-2 border-gray-300 rounded-lg p-2 text-black"
              value={text}
              onChange={(e) => set_text(e.target.value)}
            />
            <button
              className="bg-black font-bold text-white px-5 py-2 rounded-full disabled:opacity-40 hover:opacity-80"
              disabled={!text}
            >
              <ArrowRightIcon className="w-5 h-5" />
            </button>
          </form>
        </footer>
      </div>
    </>
  );
}
