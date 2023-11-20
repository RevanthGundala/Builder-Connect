import React from "react";
import Link from "next/link";

export default function Footer() {
  return (
    <footer className="pt-10 bg-cover bg-center relative mx-auto">
      <div className="container mx-auto relative z-10 text-center mb-4">
        <h2 className="text-3xl font-semibold text-white">Get Notified</h2>
        <p className="mt-4 text-gray-300">
          Want to get an email when you get a match?
        </p>
        <Link
          href="/"
          className="mt-3 inline-block bg-white text-black rounded-full py-3 px-8 hover:opacity-60"
        >
          Coming Soon!
        </Link>
        <div className="pt-8">
          <p className="text-white">Let us know what we can do better!</p>
          <p className="text-white">BuilderConnect@gmail.com </p>
        </div>
      </div>
    </footer>
  );
}
