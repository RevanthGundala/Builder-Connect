import React, { useState } from "react";
import {
  BriefcaseIcon,
  AcademicCapIcon,
  RocketLaunchIcon,
  LinkIcon,
} from "@heroicons/react/24/solid";

export default function Profile({ profile }: { profile: any }) {
  const [image_error, set_image_error] = useState(false);
  return (
    <div className="h-fit w-72 bg-white rounded-lg shadow-md">
      <div className="flex flex-row justify-center items-center pt-4 px-4">
        <img
          src={image_error ? "/images/default_user.png" : profile?.image_url}
          onError={() => set_image_error(true)}
          alt={profile?.username}
          className="w-64 h-64 object-cover rounded-full"
        />
      </div>
      <div className="flex flex-col p-4 border-b border-gray-300">
        <h2 className="text-2xl text-black">
          {profile?.username}, {profile?.age}
        </h2>
        <div className="text-gray-600 text-sm space-y-1 pt-2">
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
            <p>{profile?.employer}</p>
          </div>
          <div className="flex flex-row space-x-1">
            <BriefcaseIcon className="h-4 w-4" />
            <p>{profile?.skills}</p>
          </div>
        </div>
      </div>
      <div className="flex flex-col items-start text-sm text-gray-600 px-4 py-2 space-y-3">
        <p>{profile?.reason}</p>
        <p>{profile?.project_interests}</p>
      </div>
    </div>
  );
}
