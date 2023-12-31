import React, { useState, useEffect } from "react";
import Link from "next/link";
import { NextRouter, useRouter } from "next/router";
import { check_session } from "@/libs/functions";
import { ChevronRightIcon } from "@heroicons/react/24/solid";

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
    <nav className="flex mt-8 bg-cover relative text-white">
      <div className="flex flex-1 space-x-10 justify-center">
        <Link href="/">Home</Link>
        <Link href={sub_id !== "" ? `/profile/View` : `/SignIn`}>Profile</Link>
        <Link href={sub_id !== "" ? `/match/${sub_id}` : `/SignIn`}>
          Matches
        </Link>
        <Link href={sub_id !== "" ? `/Swipe` : `/SignIn`}>Swipe</Link>
      </div>
      <div className="flex space-x-8">
        <div className="flex rounded-full py-2 px-4 bg-white text-black opacity-70 hover:opacity-90">
          <Link href="/" className="items-center">
            Contact
          </Link>
          <ChevronRightIcon className="h-4 w-4 justify-center items-center" />
        </div>
        {sub_id === "" ? (
          <Link href="/SignIn">Sign In</Link>
        ) : (
          <button onClick={logout}>Sign Out</button>
        )}
      </div>
    </nav>
  );
}
