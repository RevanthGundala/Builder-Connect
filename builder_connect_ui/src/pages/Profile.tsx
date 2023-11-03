import ProfileComponent from "@/components/ProfileComponent";
import { NextRouter, useRouter } from "next/router";
import React, { useEffect, useState } from "react";
import Navbar from "@/components/Navbar";

export default function Profile() {
  const [profile, set_profile] = useState<any>({});
  const [image_url, set_image_url] = useState("");
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
  const [sub_id, set_sub_id] = useState("");
  const [is_connected, set_is_connected] = useState(false);
  const [is_loading, set_is_loading] = useState(false);

  async function edit_profile(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    set_is_loading(true);
    const new_profile = {
      image_url: image_url,
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
      right_swipes: profile?.right_swipes,
      left_swipes: profile?.left_swipes,
      matches: profile?.matches,
      public_fields: profile?.public_fields,
      vector_embeddings: profile?.vector_embeddings,
    };
    const url = process.env.NEXT_PUBLIC_BASE_URL + `/edit/${sub_id}`;
    const response = await fetch(url, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        accept: "application/json",
      },
      body: JSON.stringify(new_profile),
    });
    const data = await response.json();
    set_profile(data);
    set_is_loading(false);
    window.alert("Profile updated!");
  }

  useEffect(() => {
    check_session();
    view_profile();

    async function check_session() {
      try {
        const url = process.env.NEXT_PUBLIC_BASE_URL + `/get_session`;
        const res = await fetch(url, { credentials: "include" });
        const data = await res.json();
        if (data !== "Not set.") {
          set_is_connected(true);
          set_sub_id(data);
        } else {
          set_is_connected(false);
        }
        is_connected ? console.log("Connected") : console.log("Not connected");
      } catch (err) {
        console.log(err);
      }
    }

    async function view_profile() {
      if (sub_id === "") return;
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/view/${sub_id}`;
      const res = await fetch(url, { credentials: "include" });
      const data = await res.json();
      JSON.stringify(profile) === JSON.stringify(data)
        ? console.log("Same")
        : set_profile(data);
    }
  }, [is_connected, profile]);

  // TODO: Add image upload
  return (
    <>
      <Navbar is_connected={is_connected} />
      <div className="bg-gray-100 min-h-screen flex flex-col items-center">
        <div className="bg-white p-8 rounded shadow-md w-96">
          <h1 className="text-blue-500 text-center text-2xl font-semibold mb-4">
            Edit Profile
          </h1>
          <form onSubmit={edit_profile}>
            <ProfileComponent
              text={"First Name"}
              placeholder="What do you want people to call you?"
              func={set_first_name}
              required={false}
              value={profile?.first_name}
              descriptive={false}
            />
            {/* <ProfileComponent
              text={"Last Name"}
              placeholder=""
              func={set_last_name}
              required={false}
              value={profile?.last_name}
              descriptive={false}
            /> */}
            <ProfileComponent
              text={"Email"}
              placeholder=""
              func={set_email}
              required={false}
              value={profile?.email}
              descriptive={false}
            />
            <ProfileComponent
              text={"Github"}
              placeholder=""
              func={set_github}
              required={false}
              value={profile?.github}
              descriptive={false}
            />
            <ProfileComponent
              text={"Website"}
              placeholder=""
              func={set_website}
              required={false}
              value={profile?.website}
              descriptive={false}
            />
            <ProfileComponent
              text={"Age"}
              placeholder=""
              func={set_age}
              required={false}
              value={profile?.age}
              descriptive={false}
            />
            {/* <ProfileComponent
              text={"Location"}
              placeholder=""
              func={set_location}
              required={false}
              value={profile?.location}
            /> */}
            <ProfileComponent
              text={"School"}
              placeholder=""
              func={set_employer}
              required={false}
              value={profile?.employer}
              descriptive={false}
            />
            <ProfileComponent
              text={"Reason"}
              placeholder="Ex. I joined because I want to improve my resume with side projects."
              func={set_reason}
              required={false}
              value={profile?.reason}
              descriptive={true}
            />
            <ProfileComponent
              text={
                "In at most 3 sentences, describe a project you want to build."
              }
              placeholder="Ex. I want to build an AI restaurant app."
              func={set_project_interests}
              required={true}
              value={profile?.project_interests}
              descriptive={true}
            />
            <ProfileComponent
              text={"Personality Interests"}
              placeholder="Ex. I like sports, gaming, and reading."
              func={set_personality_interests}
              required={false}
              value={profile?.personality_interests}
              descriptive={true}
            />
            <ProfileComponent
              text={"Skills"}
              placeholder="Ex. Python, Java, React"
              func={set_skills}
              required={false}
              value={profile?.skills}
              descriptive={true}
            />
            <button
              type="submit"
              className="w-full bg-blue-500 text-white rounded py-2"
            >
              Save Changes
            </button>
          </form>
        </div>
      </div>
    </>
  );
}
