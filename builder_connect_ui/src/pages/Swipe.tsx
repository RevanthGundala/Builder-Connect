import ParticleBackground from "@/components/ParticleBackground";
import React, { useState, useEffect } from "react";
import Profile from "@/components/Profile";
import { ArrowRightIcon, ArrowLeftIcon } from "@heroicons/react/24/solid";
import dynamic from "next/dynamic";
import useReadSession from "@/libs/useReadSession";

export default function Swipe() {
  const { sub_id } = useReadSession();
  const [recommended_user, set_recommended_user] = useState<any>(
    "Need to fetch more users"
  );
  const Navbar = dynamic(() => import("../components/Navbar"), {
    ssr: false,
  });

  useEffect(() => {
    recommend();
  }, [sub_id]);

  async function swipe_left() {
    try {
      if (!recommended_user) return;
      const url =
        process.env.NEXT_PUBLIC_BASE_URL +
        `/swipe_left/${sub_id}/${recommended_user.sub_id}`;
      const res = await fetch(url, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          accept: "application/json",
        },
      });
      const data = await res.json();
      console.log(data);
      await recommend();
    } catch (err) {
      console.log(err);
    }
  }

  async function swipe_right() {
    try {
      if (!recommended_user) return;
      const url =
        process.env.NEXT_PUBLIC_BASE_URL +
        `/swipe_right/${sub_id}/${recommended_user.sub_id}`;
      const res = await fetch(url, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          accept: "application/json",
        },
      });
      console.log("res: ", res);
      const data = await res.json();
      console.log(data);
      await recommend();
    } catch (err) {
      console.log(err);
    }
  }

  async function recommend() {
    try {
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/recommend/${sub_id}`;
      const res = await fetch(url, { credentials: "include" });
      const data = await res.json();
      console.log(data);
      set_recommended_user(data);
    } catch (err) {
      console.log(err);
    }
  }

  return (
    <>
      <ParticleBackground />
      <Navbar />
      <div className="pt-16 bg-cover bg-center relative mx-auto">
        {recommended_user === "Need to fetch more users" ? (
          <p className="text-white py-6 text-center text-xl">No More Users!</p>
        ) : (
          <div className="pt-16 bg-cover bg-center relative">
            <div className="flex flex-row justify-center items-center">
              <Profile profile={recommended_user} />
            </div>
            <div className="mt-6 flex flex-row justify-center space-x-20 items-center p-4">
              <button
                onClick={swipe_left}
                className="bg-white rounded-full p-2 hover:opacity-70"
              >
                <ArrowLeftIcon className="h-12 w-12 text-black" />
              </button>
              <button
                onClick={swipe_right}
                className="bg-white rounded-full p-2 hover:opacity-70"
              >
                <ArrowRightIcon className="h-12 w-12 text-black" />
              </button>
            </div>
          </div>
        )}
      </div>
    </>
  );
}
