import React from "react";
import Link from "next/link";
export default function Navbar() {
  return (
    <nav className="bg-blue-500 p-4 flex flex-row">
      <header className="flex flex-row gap-24">
        <div className="p-2">Logo</div>
        <div className="p-2">Profile</div>
        <div className="p-2">Swipe</div>
      </header>
      <header className="ml-auto">
        <div className="p-2">
          <Link href="/SignIn">Sign In</Link>
        </div>
      </header>
    </nav>
  );
}
