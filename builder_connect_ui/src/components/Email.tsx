import React, { useState } from "react";
import { ArrowRightCircleIcon } from "@heroicons/react/24/solid";

export default function Email() {
  const [email, set_email] = useState("");

  async function submit_email(e: any) {
    try {
      e.preventDefault();
      const url = `${process.env.NEXT_PUBLIC_BASE_URL}/mailing_list/${email}`;
      const res = await fetch(url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          accept: "application/json",
        },
      });
      console.log("response: ", res);
      const resp = await res.text();
      console.log(resp);
      if (resp === "Added to mailing list") {
        set_email("");
        window.alert("You have been added to the mailing list!");
      }
    } catch (e) {
      console.log(e);
    }
  }

  return (
    <div className="flex justify-center items-center mb-40">
      <div className="flex flex-col w-[32rem]">
        <h2 className="text-3xl font-semibold text-white">Stay Notified</h2>
        <p className="mt-4 text-gray-300">
          Receive an email when you get a match or someone sends you a message
        </p>
        <section className="flex items-center mt-2">
          <form className="flex items-center" onSubmit={submit_email}>
            <input
              type="text"
              placeholder="YourEmail@gmail.com"
              className="flex flex-1 min-w-full h-10 rounded-lg p-2 text-black"
              value={email}
              onChange={(e) => set_email(e.target.value)}
            />
            <button
              className="font-bold text-white px-5 py-2 rounded-full disabled:opacity-40 hover:opacity-80"
              disabled={!email}
              type="submit"
            >
              <ArrowRightCircleIcon className="w-10 h-10" />
            </button>
          </form>
        </section>
      </div>
    </div>
  );
}
