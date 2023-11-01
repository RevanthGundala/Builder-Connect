import React from "react";
import Link from "next/link";
export default function Navbar({ is_connected }: { is_connected: boolean }) {
  async function sign_out() {}
  return (
    <nav className="bg-blue-500 p-4 flex flex-row">
      <header className="flex flex-row gap-24">
        <div className="p-2">
          <Link href="/">Home</Link>
        </div>
        <div className="p-2">Profile</div>
        <div className="p-2">Swipe</div>
        <div className="p-2">Matches</div>
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
          <div>
            <div className="p-2">
              <Link href="/" onClick={sign_out}>
                Sign Out
              </Link>
            </div>
            <div className="p-2">
              <Link href="/SignIn">Sign Out</Link>
            </div>
          </div>
        )}
      </header>
    </nav>
  );
}
