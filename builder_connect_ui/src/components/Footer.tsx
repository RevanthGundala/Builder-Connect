import React, { useState } from "react";
import { ArrowRightCircleIcon } from "@heroicons/react/24/solid";

export default function Footer() {
  const [email, set_email] = useState("");

  async function submit_email() {
    try {
      const url = `${process.env.NEXT_PUBLIC_BASE_URL}/mailing_list/${email}`;
      const response = await fetch(url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          accept: "application/json",
        },
      });
      const resp = await response.json();
      console.log(resp);
    } catch (e) {
      console.log(e);
    }
  }

  return (
    <footer className="pt-10 bg-cover bg-center relative mx-auto">
      <div className="container mx-auto relative z-10 text-center mb-4">
        <h2 className="text-3xl font-semibold text-white">Get Notified</h2>
        <p className="mt-4 text-gray-300">
          Receive an email when you get a match or someone sends you a message
        </p>
        <section className="flex flex-row justify-center items-center mt-2">
          <form className="flex flex-row items-center" onSubmit={submit_email}>
            <input
              type="text"
              placeholder="YourEmail@gmail.com"
              className="flex flex-row flex-1 min-w-full h-10 rounded-lg p-2 text-black"
              value={email}
              onChange={(e) => set_email(e.target.value)}
            />
            <button
              className="bg-black font-bold text-white px-5 py-2 rounded-full disabled:opacity-40 hover:opacity-80"
              disabled={!email}
              type="submit"
            >
              <ArrowRightCircleIcon className="w-10 h-10" />
            </button>
          </form>
        </section>
        <div className="pt-4">
          <p className="text-white">Let us know what we can do better!</p>
          <p className="text-white">builder.connect.network@gmail.com </p>
        </div>
      </div>
    </footer>
  );
}
