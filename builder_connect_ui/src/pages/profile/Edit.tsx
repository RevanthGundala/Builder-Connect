import ProfileComponent from "@/components/ProfileComponent";
import { NextRouter, useRouter } from "next/router";
import React, { useEffect, useState } from "react";
import { useLocalStorage } from "usehooks-ts";
import dynamic from "next/dynamic";
import ParticleBackground from "@/components/ParticleBackground";

export default function Edit() {
  const router = useRouter();
  const [profile, set_profile] = useState<any>({});
  const [image_url, set_image_url] = useState("");
  const [username, set_username] = useState("");
  const [email, set_email] = useState("");
  const [discord, set_discord] = useState("");
  const [github, set_github] = useState("");
  const [website, set_website] = useState("");
  const [age, set_age] = useState("");
  const [location, set_location] = useState("");
  const [employer, set_employer] = useState("");
  const [reason, set_reason] = useState("");
  const [project_interests, set_project_interests] = useState("");
  const [personality_interests, set_personality_interests] = useState("");
  const [skills, set_skills] = useState("");
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");
  const [is_connected, set_is_connected] = useState(false);
  const [is_loading, set_is_loading] = useState(false);
  const Navbar = dynamic(() => import("../../components/Navbar"), {
    ssr: false,
  });

  async function edit_profile(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    set_is_loading(true);
    const new_profile = {
      ...profile,
      username: username !== "" ? username : profile?.username,
      email: email !== "" ? email : profile?.email,
      github: github !== "" ? github : profile?.github,
      website: website !== "" ? website : profile?.website,
      age: age !== "" ? age : profile?.age,
      location: location !== "" ? location : profile?.location,
      employer: employer !== "" ? employer : profile?.employer,
      reason: reason !== "" ? reason : profile?.reason,
      project_interests:
        project_interests !== ""
          ? project_interests
          : profile?.project_interests,
      personality_interests:
        personality_interests !== ""
          ? personality_interests
          : profile?.personality_interests,
      skills: skills !== "" ? skills : profile?.skills,
    };
    const url = process.env.NEXT_PUBLIC_BASE_URL + `/edit/${sub_id}`;
    try {
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
      router.push(`/profile/View`);
    } catch (err) {
      console.log(err);
    }
  }

  useEffect(() => {
    view_profile();

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

  return (
    <>
      <ParticleBackground />
      <Navbar sub_id={sub_id} set_sub_id={set_sub_id} />
      <div className="pt-12 bg-cover bg-center relative mx-auto flex flex-col items-center">
        <div className="bg-white p-8 rounded shadow-md w-96">
          <h1 className="text-black text-center text-2xl mb-4">Edit Profile</h1>
          <form onSubmit={edit_profile}>
            <ProfileComponent
              text={"Username"}
              placeholder="What do you want people to call you?"
              func={set_username}
              required={true}
              value={profile?.username}
              descriptive={false}
            />
            <ProfileComponent
              text={"Email"}
              placeholder=""
              func={set_email}
              required={true}
              value={profile?.email}
              descriptive={false}
            />
            <ProfileComponent
              text={"Discord"}
              placeholder=""
              func={set_discord}
              required={false}
              value={profile?.discord}
              descriptive={false}
            />
            <ProfileComponent
              text={"Website"}
              placeholder="Personal website or github"
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
            <ProfileComponent
              text={"School"}
              placeholder=""
              func={set_employer}
              required={false}
              value={profile?.employer}
              descriptive={false}
            />
            <ProfileComponent
              text={"Skills"}
              placeholder="Ex. Python, Java, React"
              func={set_skills}
              required={false}
              value={profile?.skills}
              descriptive={true}
            />
            <ProfileComponent
              text={"In at most 3 sentences, describe your background."}
              placeholder="Ex. I am a student at the University of Waterloo. Currently, I am in my third year and I am studying computer science. I have been interested in doing a project in AI and I am looking for a partner to work with"
              func={set_reason}
              required={true}
              value={profile?.reason}
              descriptive={true}
            />
            <ProfileComponent
              text={
                "In at most 3 sentences, describe a project you want to build."
              }
              placeholder="Ex. I want to build a custom GPT that helps people do their homework."
              func={set_project_interests}
              required={true}
              value={profile?.project_interests}
              descriptive={true}
            />
            {/* <ProfileComponent
              text={"Personality Interests"}
              placeholder="Ex. I like sports, gaming, and reading."
              func={set_personality_interests}
              required={false}
              value={profile?.personality_interests}
              descriptive={true}
            /> */}
            <button
              type="submit"
              className="w-full bg-black text-white rounded py-2"
            >
              Save Changes
            </button>
          </form>
        </div>
      </div>
    </>
  );
}
