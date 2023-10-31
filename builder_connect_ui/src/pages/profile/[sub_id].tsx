import ProfileComponent from "@/components/ProfileComponent";
import { NextRouter, useRouter } from "next/router";
import React, { useEffect, useState } from "react";

export default function Profile() {
  const router = useRouter();
  const [profile, set_profile] = useState<any>({});
  const [first_name, set_first_name] = useState("");
  const [last_name, set_last_name] = useState("");
  const [email, set_email] = useState("");
  const [github, set_github] = useState("");
  const [website, set_website] = useState("");
  const [age, set_age] = useState("");
  const [location, set_location] = useState("");
  const [employer, set_employer] = useState("");
  const [reason, set_reason] = useState("");
  const [project_interests, set_project_interests] = useState("");
  const [personality_interests, set_personality_interests] = useState("");
  const [skills, set_skills] = useState("");

  async function edit_profile(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const new_profile = {
      first_name: first_name,
      last_name: last_name,
      email: email,
      github: github,
      website: website,
      age: age,
      location: location,
      employer: employer,
      reason: reason,
      project_interests: project_interests,
      personality_interests: personality_interests,
      skills: skills,
      ...profile,
    };
    // TODO: Figure out why its not updating backend profile
    const url = process.env.NEXT_PUBLIC_BASE_URL + `/edit/${profile.sub_id}`;
    const response = await fetch(url, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        accept: "application/json",
      },
      body: JSON.stringify(new_profile),
    });
    const data = await response.json();
    console.log(data);
    set_profile(data);
  }

  useEffect(() => {
    view_profile(router);

    async function view_profile(router: NextRouter) {
      const sub_id = router.asPath.split("/")[2];
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/view/${sub_id}`;
      const res = await fetch(url);
      const data = await res.json();
      set_profile(data);
    }
  }, [profile]);

  return (
    <div className="bg-gray-100 min-h-screen flex items-center justify-center">
      <div className="bg-white p-8 rounded shadow-md w-96">
        <h1 className="text-blue-500 text-center text-2xl font-semibold mb-4">
          Edit Profile
        </h1>
        <form onSubmit={edit_profile}>
          <ProfileComponent
            text={"First Name"}
            func={set_first_name}
            required={false}
          />
          <ProfileComponent
            text={"Last Name"}
            func={set_last_name}
            required={false}
          />
          <ProfileComponent text={"Email"} func={set_email} required={false} />
          <ProfileComponent
            text={"Github"}
            func={set_github}
            required={false}
          />
          <ProfileComponent
            text={"Website"}
            func={set_website}
            required={false}
          />
          <ProfileComponent text={"Age"} func={set_age} required={false} />
          <ProfileComponent
            text={"Location"}
            func={set_location}
            required={false}
          />
          <ProfileComponent
            text={"School"}
            func={set_employer}
            required={false}
          />
          <ProfileComponent text={"Reason"} func={set_reason} required={true} />
          <ProfileComponent
            text={"Project Interests"}
            func={set_project_interests}
            required={true}
          />
          <ProfileComponent
            text={"Personality Interests"}
            func={set_personality_interests}
            required={false}
          />
          <ProfileComponent text={"Skills"} func={set_skills} required={true} />
          <button
            type="submit"
            className="w-full bg-blue-500 text-white rounded py-2"
          >
            Save Changes
          </button>
        </form>
      </div>
    </div>
  );
}
