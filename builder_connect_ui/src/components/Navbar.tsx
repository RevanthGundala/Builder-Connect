import React, { useState, useEffect } from "react";
import Link from "next/link";
import { NextRouter, useRouter } from "next/router";

export default function Navbar({ is_connected }: { is_connected: boolean }) {
  const [sub_id, set_sub_id] = useState("");
  const router = useRouter();

  async function logout() {
    const url = process.env.NEXT_PUBLIC_BASE_URL + "/logout";
    const response = await fetch(url, { credentials: "include" });
    const data = await response.json();
    console.log(data);
  }

  useEffect(() => {
    set_sub_id(router.asPath.split("/")[2]);
  }, [sub_id]);

  return (
    <nav className="bg-blue-500 p-4 flex flex-row">
      <header className="flex flex-row gap-24">
        <div className="p-2">
          <Link href="/">Home</Link>
        </div>
        {is_connected ? (
          <>
            <div className="p-2">
              <Link href={`/view/${sub_id}`}>Profile</Link>
            </div>
            <div className="p-2">
              <Link className="p-2" href={`/matches/${sub_id}`}>
                Matches
              </Link>
            </div>
            <div className="p-2">
              <Link className="p-2" href={`/recommend/${sub_id}`}>
                Swipe
              </Link>
            </div>
          </>
        ) : (
          <div>
            <Link className="p-2" href={`/SignIn`}>
              Profile
            </Link>
            <Link className="p-2" href={`/SignIn`}>
              Matches
            </Link>
            <Link className="p-2" href={`/SignIn`}>
              Swipe
            </Link>
          </div>
        )}
        <div className="p-2">
          <Link href="/About">About</Link>
        </div>
      </header>
      <header className="ml-auto">
        {!is_connected ? (
          <div className="p-2">
            <Link href="/SignIn">Sign In</Link>
          </div>
        ) : (
          <div className="p-2">
            <Link onClick={logout} href="/">
              Sign Out
            </Link>
          </div>
        )}
      </header>
    </nav>
  );
}
