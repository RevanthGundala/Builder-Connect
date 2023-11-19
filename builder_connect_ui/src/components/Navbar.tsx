import React, { useState, useEffect } from "react";
import Link from "next/link";
import { NextRouter, useRouter } from "next/router";
import { useLocalStorage } from "usehooks-ts";
import { check_session } from "@/functions/check_session";

export default function Navbar() {
  const router = useRouter();
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");
  const [isClient, set_is_client] = useState(false);

  async function logout() {
    const url = process.env.NEXT_PUBLIC_BASE_URL + "/logout";
    const response = await fetch(url, { credentials: "include" });
    const data = await response.json();
    const id = await check_session();
    id ? set_sub_id(id) : set_sub_id("");
    router.push("/");
    console.log(data);
  }

  useEffect(() => {
    set_is_client(true);
    if (sub_id === "") {
      let updating = true;
      let sub = setInterval(async () => {
        let id = await check_session();
        if (id) set_sub_id(id);
      }, 3000);
      return () => {
        clearInterval(sub);
        updating = false;
      };
    }
  }, [sub_id]);

  console.log("sub_id: ", sub_id);
  return isClient ? (
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
  ) : (
    <div>Loading</div>
  );
}
