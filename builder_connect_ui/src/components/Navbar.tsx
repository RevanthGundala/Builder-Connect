import React, { useState, useEffect } from "react";
import Link from "next/link";
import { NextRouter, useRouter } from "next/router";
import { useLocalStorage } from "usehooks-ts";
import { check_session } from "@/libs/functions";

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

  return isClient ? (
    <nav className="p-4 flex flex-row space-x-16 items-center text-white bg-cover bg-center relative justify-center">
      <Link href="/">Home</Link>
      {sub_id !== "" ? (
        <>
          <Link href={`/profile/View`}>Profile</Link>
          <Link href={`/match/${sub_id}`}>Matches</Link>
          <Link href={`/Swipe`}>Swipe</Link>
        </>
      ) : (
        <>
          <Link href={`/SignIn`}>Profile</Link>
          <Link href={`/SignIn`}>Matches</Link>
          <Link href={`/SignIn`}>Swipe</Link>
        </>
      )}
      {sub_id === "" ? (
        <div className="p-2">
          <Link href="/SignIn">Sign In</Link>
        </div>
      ) : (
        <div className="p-2">
          <button onClick={logout}>Sign Out</button>
        </div>
      )}
    </nav>
  ) : (
    <div>Loading</div>
  );
}
