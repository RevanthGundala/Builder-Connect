import React, { useState, useEffect } from "react";
import Link from "next/link";
import dynamic from "next/dynamic";
import { useLocalStorage } from "usehooks-ts";
import ParticleBackground from "@/components/ParticleBackground";
import { view_profile } from "@/libs/functions";
import Profile from "@/components/Profile";

export default function View() {
  const [profile, set_profile] = useState<any>({});
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");
  const [alert_shown, set_alert_shown] = useState(false);
  const Navbar = dynamic(() => import("../../components/Navbar"), {
    ssr: false,
  });

  useEffect(() => {
    view();
    async function view() {
      const profile_data = await view_profile(sub_id, profile);
      set_profile(profile_data);
    }
  }, [profile]);

  useEffect(() => {
    if (
      !alert_shown &&
      (profile?.project_interests === "" || profile?.reason === "")
    ) {
      window.alert("You must fill out your profile before you can swipe!");
      set_alert_shown(true);
    }
  }, [alert_shown, profile]);

  return (
    <>
      <ParticleBackground />
      <Navbar sub_id={sub_id} set_sub_id={set_sub_id} />
      <div className="pt-16 bg-cover bg-center relative mx-auto">
        <div className="flex flex-col items-center justify-center mt-10 container mx-auto relative z-10 text-center">
          <Profile profile={profile} />
          <Link
            href={`/profile/Edit`}
            className="mt-6 inline-block bg-white text-black rounded-full py-3 px-8 hover:opacity-60 animate-pulse"
          >
            Edit Profile
          </Link>
        </div>
      </div>
    </>
  );
}
