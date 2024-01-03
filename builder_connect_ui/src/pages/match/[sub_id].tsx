import ParticleBackground from "@/components/ParticleBackground";
import React, { useEffect, useMemo, useState } from "react";
import Sidebar from "@/components/Sidebar";
import Profile from "@/components/Profile";
import MessageComponent from "@/components/MessageComponent";
import Link from "next/link";
import { ArrowLeftIcon } from "@heroicons/react/24/solid";
import useReadSession from "@/libs/useReadSession";
import useProfile from "@/libs/useProfile";

export default function Messages() {
  const [profile, set_profile] = useProfile(undefined);
  const [match_sub_id, set_match_sub_id] = useState("");
  const [match_profile, set_match_profile] = useProfile(match_sub_id);
  const { sub_id } = useReadSession();
  const [match_room_id, set_match_room_id] = useState<string>("");
  const [all_messages, set_all_messages] = useState(new Map());
  const [room_to_last_message, set_room_to_last_message] = useState(new Map());

  useEffect(() => {
    const controller = new AbortController();
    fetch_data();

    async function fetch_data() {
      if (profile) {
        for (const room of profile.matches) {
          const room_id = room.room_id;
          const messages = await fetch_room_data(room_id);
          set_all_messages((prev) => {
            return new Map(prev.set(room_id, messages));
          });
          set_room_to_last_message((prev) => {
            const last_message = messages.slice(-1)[0];
            return new Map(prev.set(room_id, last_message));
          });
        }
      }
    }

    async function fetch_room_data(room_id: string) {
      if (room_id === "") return;
      try {
        const url = `${process.env.NEXT_PUBLIC_BASE_URL}/messages/${room_id}`;
        const response = await fetch(url);
        const room_data = await response.json();
        return room_data;
      } catch (e) {
        console.log(e);
      }
    }

    console.log("room_id: ", match_room_id);
    return () => controller.abort();
  }, [profile, match_profile, match_room_id]);

  return (
    <>
      <ParticleBackground />
      <div className="bg-cover bg-center relative text-white">
        <div className="grid grid-cols-9 min-h-screen">
          <div className="col-span-2 border-r border-gray-300">
            <div className="py-2 px-1">
              <Link href="/profile/View">
                <ArrowLeftIcon className="h-8 w-8 text-white" />
              </Link>
            </div>
            <Sidebar
              profile={profile}
              match_sub_id={match_sub_id}
              set_match_sub_id={set_match_sub_id}
              set_match_room_id={set_match_room_id}
              room_to_last_message={room_to_last_message}
              set_room_to_last_message={set_room_to_last_message}
            />
          </div>
          <div className="md:col-span-7 lg:col-span-5 border-r border-gray-300">
            {match_profile &&
            JSON.stringify(profile) !== JSON.stringify(match_profile) ? (
              <MessageComponent
                match_profile={match_profile}
                sub_id={sub_id}
                room_id={match_room_id}
                all_messages={all_messages}
              />
            ) : (
              <div className="flex flex-col items-center justify-center h-full">
                <p className="text-2xl">Click on a conversation to begin!</p>
              </div>
            )}
          </div>
          <div className="pt-8 mx-auto hidden lg:col-span-2 lg:inline">
            <Profile profile={match_profile ?? profile} />
          </div>
        </div>
      </div>
    </>
  );
}
