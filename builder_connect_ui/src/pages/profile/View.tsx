import React, { useState, useEffect } from "react";
import Link from "next/link";
import dynamic from "next/dynamic";
import { useLocalStorage } from "usehooks-ts";
import ParticleBackground from "@/components/ParticleBackground";
import Profile from "@/components/Profile";
import useProfile from "@/libs/useProfile";
// import Navbar from "@/components/Navbar";
export default function View() {
  const [alert_shown, set_alert_shown] = useState(false);
  const [profile, set_profile] = useProfile();

  const Navbar = dynamic(() => import("../../components/Navbar"), {
    ssr: false,
  });

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
      <Navbar />
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
