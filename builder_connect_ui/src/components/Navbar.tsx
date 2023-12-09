import React, { useState, useEffect } from "react";
import Link from "next/link";
import { NextRouter, useRouter } from "next/router";
import { check_session } from "@/libs/functions";

export default function Navbar({
  sub_id,
  set_sub_id,
}: {
  sub_id: string;
  set_sub_id: React.Dispatch<React.SetStateAction<any>>;
}) {
  const router = useRouter();

  async function logout() {
    try {
      const url = process.env.NEXT_PUBLIC_BASE_URL + "/logout";
      const response = await fetch(url, { credentials: "include" });
      const data = await response.json();
      const id = await check_session();
      id ? set_sub_id(id) : set_sub_id("");
      router.push("/");
      console.log(data);
    } catch (e) {
      console.log(e);
    }
  }

  useEffect(() => {
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

  return (
    <nav className="p-4 flex flex-row space-x-16 items-center text-white bg-cover bg-center relative justify-center">
      <Link href="/">Home</Link>
      <Link href={sub_id !== "" ? `/profile/View` : `/SignIn`}>Profile</Link>
      <Link href={sub_id !== "" ? `/match/${sub_id}` : `/SignIn`}>Matches</Link>
      <Link href={sub_id !== "" ? `/Swipe` : `/SignIn`}>Swipe</Link>
      <div className="p-2">
        {sub_id === "" ? (
          <Link href="/SignIn">Sign In</Link>
        ) : (
          <button onClick={logout}>Sign Out</button>
        )}
      </div>
    </nav>
  );
}
