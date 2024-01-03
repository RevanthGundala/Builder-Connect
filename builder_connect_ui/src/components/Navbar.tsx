import React, { useState, useEffect } from "react";
import Link from "next/link";
import { NextRouter, useRouter } from "next/router";
import { ChevronRightIcon } from "@heroicons/react/24/solid";
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

  return (
    <nav className="flex mt-8 bg-cover relative border-b border-gray-600 pb-6 text-white items-center">
      <div className="flex flex-1 space-x-10 justify-start px-12">
        <Link href="/">Home</Link>
        <Link href={sub_id !== "" ? `/profile/View` : `/SignIn`}>Profile</Link>
        <Link href={sub_id !== "" ? `/match/${sub_id}` : `/SignIn`}>
          Matches
        </Link>
        <Link href={sub_id !== "" ? `/Swipe` : `/SignIn`}>Swipe</Link>
      </div>
      <div className="flex space-x-2 text-black p-2 pl-3 items-center rounded-full bg-white mr-10 hover:opacity-70">
        {sub_id === "" ? (
          <Link href="/SignIn">Sign In</Link>
        ) : (
          <button onClick={logout}>Sign Out</button>
        )}
        <ChevronRightIcon className="w-3 h-3" />
      </div>
    </nav>
  );
}
