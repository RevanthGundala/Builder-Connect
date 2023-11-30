import React, { useEffect, useState } from "react";
import { PhotoIcon, ArrowRightIcon } from "@heroicons/react/24/solid";
import Room from "@/components/Conversation";
import Conversation from "@/components/Conversation";
import useConversations from "@/libs/useConversation";
import useWebsocket from "@/libs/useWebsocket";

export default function MessageComponent({
  profile,
  match_profile,
  sub_id,
  room_id,
}: {
  profile: any;
  match_profile: any;
  sub_id: string;
  room_id: any;
}) {
  const [text, set_text] = useState("");
  const [is_typing, set_is_typing] = useState(false);
  const [is_loading, messages, set_messages, fetch_conversations] =
    useConversations("");

  function handle_typing(mode: string) {
    mode === "IN" ? set_is_typing(true) : set_is_typing(false);
  }

  function handle_message(msg: string, user_sub_id: string) {
    set_messages((prevMessages: any) => {
      const item = { content: msg, user_sub_id: user_sub_id };
      set_text("");
      return [...prevMessages, item];
    });
  }

  function on_message(data: any) {
    try {
      let message_data = JSON.parse(data);
      //   console.log("messgdata: ", message_data);
      if (message_data?.chat_type == "TYPING") {
        handle_typing(message_data?.value[0]);
      } else if (message_data?.chat_type == "TEXT") {
        handle_message(message_data?.value[0], message_data?.user_sub_id);
      }
    } catch (err) {
      console.log(err);
    }
  }

  const send_message = useWebsocket(on_message, room_id);

  function submit_message(e: any) {
    e.preventDefault();
    if (!room_id) {
      alert("Please select room");
    } else {
      const data = {
        room_id: room_id,
        user_id: sub_id,
        chat_type: "TEXT",
        content: text,
      };
      send_message(JSON.stringify(data));
      handle_message(text, sub_id);
    }
  }

  useEffect(() => {
    fetch_conversations(room_id);
  }, [room_id]);

  return (
    <>
      <div className="flex flex-col bg-gray-200 h-5/6 max-h-full w-5/6 mx-auto my-8">
        <header className="bg-gray-300 border-b border-gray-400 py-6 px-2">
          <div className="flex flex-col space-y-1 items-center">
            <img
              src={match_profile?.image_url}
              alt={match_profile?.username}
              className="w-6 h-6 object-cover rounded-full"
            />
            <div className="text-md text-black">
              <p>{match_profile?.username}</p>
            </div>
          </div>
        </header>
        <main className="flex flex-col flex-1 border-b border-gray-400 overflow-auto">
          <Conversation
            messages={messages}
            sub_id={sub_id}
            profile={profile}
            match_profile={match_profile}
          />
        </main>
        <footer>
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
