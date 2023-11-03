import Navbar from "@/components/Navbar";
import { NextRouter, useRouter } from "next/router";
import React, { useState, useEffect } from "react";

export default function Swipe() {
  const [is_connected, set_is_connected] = useState(false);
  const [sub_id, set_sub_id] = useState("");
  const [recommended_user, set_recommended_user] = useState<any>(
    "Need to fetch more users"
  );

  useEffect(() => {
    check_session();
    recommend();

    async function check_session() {
      try {
        const url = process.env.NEXT_PUBLIC_BASE_URL + `/get_session`;
        const res = await fetch(url, { credentials: "include" });
        const data = await res.json();
        if (data !== "Not set.") {
          set_is_connected(true);
          set_sub_id(data);
        } else {
          set_is_connected(false);
        }
        is_connected ? console.log("Connected") : console.log("Not connected");
      } catch (err) {
        console.log(err);
      }
    }
  }, [is_connected]);

  async function swipe_left() {
    try {
      if (!recommended_user) return;
      const url =
        process.env.NEXT_PUBLIC_BASE_URL +
        `/swipe_left/${sub_id}/${recommended_user.sub_id}`;
      const res = await fetch(url, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          accept: "application/json",
        },
      });
      const data = await res.json();
      console.log(data);
      await recommend();
    } catch (err) {
      console.log(err);
    }
  }

  async function swipe_right() {
    try {
      if (!recommended_user) return;
      const url =
        process.env.NEXT_PUBLIC_BASE_URL +
        `/swipe_right/${sub_id}/${recommended_user.sub_id}`;
      const res = await fetch(url, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          accept: "application/json",
        },
      });
      const data = await res.json();
      console.log(data);
      await recommend();
    } catch (err) {
      console.log(err);
    }
  }

  async function recommend() {
    try {
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/recommend/${sub_id}`;
      const res = await fetch(url, { credentials: "include" });
      const data = await res.json();
      console.log(data);
      set_recommended_user(data);
    } catch (err) {
      console.log(err);
    }
  }

  return (
    <>
      <Navbar is_connected={is_connected} />
      {recommended_user === "Need to fetch more users" ? (
        <div className="bg-gray-100 min-h-screen">
          <p className="text-black p-4">{recommended_user}</p>
        </div>
      ) : (
        <div className="bg-gray-100 min-h-screen">
          <div className="flex flex-col items-center justify-center h-screen">
            <div className="w-72 h-96 bg-white rounded-lg shadow-md">
              <img
                src={recommended_user?.imageUrl}
                alt={recommended_user?.name}
                className="w-full h-64 object-cover rounded-t-lg"
              />
              <div className="p-4">
                <h2 className="text-2xl font-semibold">
                  <p className="text-black">
                    {recommended_user?.first_name}, {recommended_user?.age}
                  </p>
                </h2>
                <p className="text-gray-600">
                  {recommended_user?.project_interests}
                </p>
              </div>
            </div>
            <div className="mt-4">
              <button
                onClick={swipe_left}
                className="bg-red-500 text-white px-4 py-2 rounded-lg mr-4"
              >
                No
              </button>
              <button
                onClick={swipe_right}
                className="bg-green-500 text-white px-4 py-2 rounded-lg"
              >
                Yes
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
