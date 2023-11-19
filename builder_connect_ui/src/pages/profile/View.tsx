import React, { useState, useEffect } from "react";
import Navbar from "@/components/Navbar";
import Link from "next/link";
import {
  BriefcaseIcon,
  AcademicCapIcon,
  RocketLaunchIcon,
  LinkIcon,
} from "@heroicons/react/24/solid";
import { useLocalStorage } from "usehooks-ts";

export default function View() {
  const [profile, set_profile] = useState<any>({});
  const [image_url, set_image_url] = useState("");
  const [image, set_image] = useState<any>();
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");

  async function fetch_image() {
    const res = await fetch(image_url);
    const imageBlob = await res.blob();
    const imageObjectURL = URL.createObjectURL(imageBlob);
    set_image(imageObjectURL);
  }

  useEffect(() => {
    view_profile();

    async function view_profile() {
      if (sub_id === "") return;
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/view/${sub_id}`;
      const res = await fetch(url, { credentials: "include" });
      const data = await res.json();
      console.log("data: ", data);
      if (image_url !== data.image_url) {
        set_image_url(data.image_url);
        await fetch_image();
      }
      if (JSON.stringify(profile) !== JSON.stringify(data)) {
        set_profile(data);
      }
    }
  }, [profile]);
  return (
    <>
      <Navbar />
      {sub_id === "" ? (
        <div>Loading...</div>
      ) : (
        <div className="bg-white min-h-screen">
          <div className="flex flex-col items-center justify-center mt-10">
            <div className="w-fit h-fit bg-white rounded-lg shadow-md">
              <img
                src={profile.image_url}
                alt={profile.username}
                className="w-full h-64 object-cover rounded-t-lg"
              />
              <div className="flex flex-col p-4 space-y-3 border-b border-gray-300">
                <h2 className="text-2xl font-semibold">
                  {profile.username} , {profile.age}
                </h2>
                <div className="text-gray-600 text-sm">
                  {profile?.website ? (
                    <div className="flex flex-row space-x-1">
                      <LinkIcon className="h-4 w-4" />
                      <p>{profile.website}</p>
                    </div>
                  ) : (
                    <div></div>
                  )}
                  <div className="flex flex-row space-x-1">
                    <AcademicCapIcon className="h-4 w-4" />
                    <p>{profile.employer}</p>
                  </div>
                  <div className="flex flex-row space-x-1">
                    <BriefcaseIcon className="h-4 w-4" />
                    <p>{profile.skills}</p>
                  </div>
                </div>
              </div>
              <div className="flex flex-col items-start text-sm text-gray-600 px-4 py-2">
                <p>{profile.reason}</p>
                <p>{profile.project_interests}</p>
              </div>
            </div>
            <Link
              href={`/profile/Edit`}
              className="bg-blue-500 rounded-large text-md text-white p-2 flex justify-center w-fit mt-2"
            >
              Edit Profile
            </Link>
          </div>
        </div>
      )}
    </>
  );
}
