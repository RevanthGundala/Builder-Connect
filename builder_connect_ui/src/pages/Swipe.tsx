import Navbar from "@/components/Navbar";
import ParticleBackground from "@/components/ParticleBackground";
import { NextRouter, useRouter } from "next/router";
import React, { useState, useEffect } from "react";
import { useLocalStorage } from "usehooks-ts";
import {
  BriefcaseIcon,
  AcademicCapIcon,
  RocketLaunchIcon,
  LinkIcon,
} from "@heroicons/react/24/solid";

export default function Swipe() {
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");
  const [recommended_user, set_recommended_user] = useState<any>(
    "Need to fetch more users"
  );

  useEffect(() => {
    recommend();
  }, []);

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
          <p className="text-white p-4">{recommended_user}</p>
        ) : (
          <div className="pt-16 bg-cover bg-center relative mx-auto">
            <div className="flex flex-col items-center justify-center mt-10 container mx-auto relative z-10 text-center">
              <div className="w-fit h-fit bg-white rounded-lg shadow-md">
                <div className="flex flex-row justify-center items-center pt-4">
                  <img
                    src={recommended_user.image_url}
                    alt={recommended_user.username}
                    className="w-64 h-64 object-cover rounded-full"
                  />
                </div>
                <div className="flex flex-col p-4 border-b border-gray-300">
                  <h2 className="text-2xl">
                    {recommended_user.username}, {recommended_user.age}
                  </h2>
                  <div className="text-gray-600 text-sm space-y-1 pt-2">
                    {recommended_user?.website ? (
                      <div className="flex flex-row space-x-1">
                        <LinkIcon className="h-4 w-4" />
                        <p>{recommended_user.website}</p>
                      </div>
                    ) : (
                      <div></div>
                    )}
                    <div className="flex flex-row space-x-1">
                      <AcademicCapIcon className="h-4 w-4" />
                      <p>{recommended_user.employer}</p>
                    </div>
                    <div className="flex flex-row space-x-1">
                      <BriefcaseIcon className="h-4 w-4" />
                      <p>{recommended_user.skills}</p>
                    </div>
                  </div>
                </div>
                <div className="flex flex-col items-start text-sm text-gray-600 px-4 py-2 space-y-3">
                  <p>{recommended_user.reason}</p>
                  <p>{recommended_user.project_interests}</p>
                </div>
              </div>
              <div className="mt-4">
                <button
                  onClick={swipe_left}
                  className="bg-red-500 text-white px-4 py-2 rounded-lg mr-4"
                >
                  No
                </button>
                <button
                  onClick={swipe_right}
                  className="bg-green-500 text-white px-4 py-2 rounded-lg"
                >
                  Yes
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </>
  );
}
