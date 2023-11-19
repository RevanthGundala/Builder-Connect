import React, { useState, useEffect } from "react";
import Link from "next/link";
import { NextRouter, useRouter } from "next/router";
import { useLocalStorage } from "@uidotdev/usehooks";

export default function Navbar({
  sub_id,
  set_sub_id,
}: {
  sub_id: string;
  set_sub_id: any;
}) {
  const router = useRouter();
  //const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");

  async function logout() {
    const url = process.env.NEXT_PUBLIC_BASE_URL + "/logout";
    const response = await fetch(url, { credentials: "include" });
    const data = await response.json();
    router.push("/");
    console.log(data);
  }

  // TODO: change sub_id to localstorage
  useEffect(() => {
    if (sub_id === "") {
      let updating = true;
      let sub = setInterval(() => {
        check_session();
      }, 3000);
      return () => {
        clearInterval(sub);
        updating = false;
      };
    }

    async function check_session() {
      try {
        const url = process.env.NEXT_PUBLIC_BASE_URL + `/get_session`;
        const res = await fetch(url, { credentials: "include" });
        const data = await res.json();
        data === "Not set." ? set_sub_id("") : set_sub_id(data);
      } catch (err) {
        console.log(err);
      }
    }
  }, [sub_id]);

  console.log("sub_id: ", sub_id);

  return (
    <nav className="bg-blue-500 p-4 flex flex-row text-white">
      <header className="flex flex-row gap-24">
        <div className="p-2">
          <Link href="/">Home</Link>
        </div>
        {sub_id !== "" ? (
          <>
            <div className="p-2">
              <Link href={`/profile/View`}>Profile</Link>
            </div>
            <div className="p-2">
              <Link href={`/Match`}>Matches</Link>
            </div>
            <div className="p-2">
              <Link href={`/Swipe`}>Swipe</Link>
            </div>
          </>
        ) : (
          <>
            <div className="p-2">
              <Link href={`/SignIn`}>Profile</Link>
            </div>
            <div className="p-2">
              <Link href={`/SignIn`}>Matches</Link>
            </div>
            <div className="p-2">
              <Link href={`/SignIn`}>Swipe</Link>
            </div>
          </>
        )}
      </header>
      <header className="ml-auto">
        {sub_id === "" ? (
          <div className="p-2">
            <Link href="/SignIn">Sign In</Link>
          </div>
        ) : (
          <div className="p-2">
            <button onClick={logout}>Sign Out</button>
          </div>
        )}
      </header>
    </nav>
  );
}
