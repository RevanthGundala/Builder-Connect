import React, { useState } from "react";
import { PhotoIcon, ArrowRightIcon } from "@heroicons/react/24/solid";

export default function MessageComponent({ profile }: { profile: any }) {
  const [text, set_text] = useState("");
  return (
    <>
      <div className="flex flex-col bg-gray-200 h-5/6 w-5/6 mx-auto my-8">
        <header className="bg-gray-300 border-b border-gray-400 py-6 px-2">
          <div className="flex flex-row">
            <img
              src={profile?.image_url}
              alt={profile?.username}
              className="w-6 h-6 object-cover rounded-full"
            />
            <div className="flex flex-col text-sm text-black">
              <p>Conversation with {profile?.username}</p>
            </div>
          </div>
        </header>
        <main className="flex flex-col flex-1 border-b border-gray-400">
          Messages
        </main>
        <footer>
          <form className="flex flex-row items-center space-x-2 p-4">
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
