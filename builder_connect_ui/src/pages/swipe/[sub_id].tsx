import { NextRouter, useRouter } from "next/router";
import React, { useState, useEffect } from "react";

const data = [
  {
    id: 1,
    name: "User 1",
    age: 25,
    bio: "Fun-loving adventurer. Enjoys hiking and traveling.",
    imageUrl: "user1.jpg",
  },
  {
    id: 2,
    name: "User 2",
    age: 28,
    bio: "Coffee lover and bookworm. Looking for a partner in crime.",
    imageUrl: "user2.jpg",
  },
  // Add more user data here
];

export default function Swipe() {
  const router = useRouter();
  const [recommended_user, set_recommended_user] = useState<any>({});

  async function swipe_left(router: NextRouter) {
    try {
      const sub_id = router.asPath.split("/")[2];
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/swipe_left/${sub_id}`;
      const res = await fetch(url, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          accept: "application/json",
        },
      });
      const data = await res.json();
      console.log(data);
      await recommend(router);
    } catch (err) {
      console.log(err);
    }
  }

  async function swipe_right(router: NextRouter) {
    try {
      const sub_id = router.asPath.split("/")[2];
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/swipe_right/${sub_id}`;
      const res = await fetch(url, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          accept: "application/json",
        },
      });
      const data = await res.json();
      console.log(data);
      await recommend(router);
    } catch (err) {
      console.log(err);
    }
  }

  async function recommend(router: NextRouter) {
    try {
      const sub_id = router.asPath.split("/")[2];
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/recommend/${sub_id}`;
      const res = await fetch(url, {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          accept: "application/json",
        },
      });
      const data = await res.json();
      console.log(data);
      set_recommended_user(data);
    } catch (err) {
      console.log(err);
    }
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen">
      <div className="w-72 h-96 bg-white rounded-lg shadow-md">
        <img
          src={recommended_user?.imageUrl}
          alt={recommended_user?.name}
          className="w-full h-64 object-cover rounded-t-lg"
        />
        <div className="p-4">
          <h2 className="text-2xl font-semibold">
            {recommended_user?.name}, {recommended_user?.age}
          </h2>
          <p className="text-gray-600">{recommended_user?.bio}</p>
        </div>
      </div>
      <div className="mt-4">
        <button
          onClick={() => swipe_left(router)}
          className="bg-red-500 text-white px-4 py-2 rounded-lg mr-4"
        >
          No
        </button>
        <button
          onClick={() => swipe_right(router)}
          className="bg-green-500 text-white px-4 py-2 rounded-lg"
        >
          Yes
        </button>
      </div>
    </div>
  );
}
