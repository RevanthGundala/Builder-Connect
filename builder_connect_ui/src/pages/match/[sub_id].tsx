import ParticleBackground from "@/components/ParticleBackground";
import React, { useEffect, useState } from "react";
import Navbar from "@/components/Navbar";
import Sidebar from "@/components/Sidebar";
import { view_profile } from "@/libs/functions";
import { useLocalStorage } from "usehooks-ts";
import { useRouter } from "next/router";
import Profile from "@/components/Profile";
import MessageComponent from "@/components/MessageComponent";
import Link from "next/link";
import { ArrowLeftIcon } from "@heroicons/react/24/solid";

export default function Messages() {
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");
  const [profile, set_profile] = useState(null);
  const [match_profile, set_match_profile] = useState<any>(null);
  const [match_room_id, set_match_room_id] = useState<any>(null);

  return (
    <>
      <ParticleBackground />
      <div className="bg-cover bg-center relative text-white">
        <div className="grid grid-cols-9 min-h-screen overflow-hidden">
          <div className="col-span-2 border-r border-gray-300">
            <div className="py-2 px-1">
              <Link href="/">
                <ArrowLeftIcon className="h-8 w-8 text-white" />
              </Link>
            </div>
            <Sidebar
              sub_id={sub_id}
              profile={profile}
              set_profile={set_profile}
              set_match_profile={set_match_profile}
              set_match_room_id={set_match_room_id}
            />
          </div>
          <div className="md:col-span-7 lg:col-span-5 border-r border-gray-300">
            {match_profile ? (
              <MessageComponent
                profile={profile}
                match_profile={match_profile}
                sub_id={sub_id}
                room_id={match_room_id}
              />
            ) : (
              <div className="flex flex-col items-center justify-center h-full">
                <p className="text-2xl">Click on a conversation to begin!</p>
              </div>
            )}
          </div>
          <div className="pt-8 mx-auto hidden lg:col-span-2 lg:inline">
            <Profile profile={match_profile ? match_profile : profile} />
          </div>
        </div>
      </div>
    </>
  );
}
