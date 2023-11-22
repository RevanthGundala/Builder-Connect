import ParticleBackground from "@/components/ParticleBackground";
import React, { useEffect, useState } from "react";
import Navbar from "@/components/Navbar";
import Sidebar from "@/components/Sidebar";
import { view_profile } from "@/calls/functions";
import { useLocalStorage } from "usehooks-ts";
import { useRouter } from "next/router";
import Profile from "@/components/Profile";
import MessageComponent from "@/components/MessageComponent";
import Link from "next/link";
import { ArrowLeftIcon } from "@heroicons/react/24/solid";

export default function Messages() {
  const [profile, set_profile] = useState<any>(undefined);
  const [matches, set_matches] = useState<any[]>([]);
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");
  const [match_sub_id, set_match_sub_id] = useState("");
  const [match_profile, set_match_profile] = useState<any>(undefined);
  const router = useRouter();

  async function get_profile(id = sub_id) {
    const profile_data = await view_profile(id, profile);
    id === sub_id ? set_profile(profile_data) : set_match_profile(profile_data);
    if (id === sub_id) {
      let matches = await Promise.all(
        profile_data?.matches.map(async (match_id: number) => {
          const url = process.env.NEXT_PUBLIC_BASE_URL + `/view/${match_id}`;
          const res = await fetch(url);
          const match = await res.json();
          return match;
        })
      );
      set_matches(matches);
    }
  }

  useEffect(() => {
    get_profile();
    if (sub_id !== router.query.sub_id) router.push("/Error");
  }, []);

  useEffect(() => {
    get_profile(match_sub_id);
  }, [match_sub_id]);

  return (
    <>
      <ParticleBackground />
      <div className="bg-cover bg-center relative text-white">
        <div className="grid grid-cols-9 min-h-screen overflow-auto">
          <div className="col-span-2 border-r border-gray-300">
            <div className="py-2 px-1">
              <Link href="/">
                <ArrowLeftIcon className="h-8 w-8 text-white" />
              </Link>
            </div>
            <Sidebar
              matches={matches}
              sub_id={match_sub_id}
              set_sub_id={set_match_sub_id}
            />
          </div>
          <div className="md:col-span-7 lg:col-span-5 border-r border-gray-300">
            <MessageComponent profile={match_profile} />
          </div>
          <div className="pt-8 mx-auto hidden lg:col-span-2 lg:inline">
            <Profile profile={match_profile} />
          </div>
        </div>
      </div>
    </>
  );
}
