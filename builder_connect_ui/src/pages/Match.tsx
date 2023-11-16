import Navbar from "@/components/Navbar";
import { NextRouter, useRouter } from "next/router";
import React, { useEffect, useState } from "react";
const matchesData = [
  {
    id: 1,
    name: "Alice",
    age: 28,
    bio: "Loves hiking and traveling.",
    imageUrl: "alice.jpg",
  },
  {
    id: 2,
    name: "Bob",
    age: 24,
    bio: "Passionate about photography.",
    imageUrl: "bob.jpg",
  },
  // Add more match data here
];

export default function Matches() {
  const router = useRouter();
  const [is_connected, set_is_connected] = useState(false);
  const [sub_id, set_sub_id] = useState("");
  const [matches, set_matches] = useState<any>([]);

  useEffect(() => {
    check_session();
    view_matches();

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
      } catch (err) {
        console.log(err);
      }
    }

    async function view_matches() {
      if (sub_id === "") return;
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/matches/${sub_id}`;
      const res = await fetch(url);
      const match_ids = await res.json();
      console.log(match_ids);
      match_ids.forEach(async (id: number) => {
        const url = process.env.NEXT_PUBLIC_BASE_URL + `/view/${id}`;
        const res = await fetch(url);
        const data = await res.json();
        matches.push(data);
      });
      console.log(matches);
    }
  }, [matches, is_connected]);

  return (
    <>
      <Navbar is_connected={is_connected} />

      <div className="bg-gray-100 p-4 min-h-screen">
        <h1 className="text-2xl text-black font-semibold mb-4">Your Matches</h1>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {matches.length > 0 ? (
            matches.map((match: any) => (
              <div
                key={match.sub_id}
                className="bg-white rounded shadow-md p-4 cursor-pointer transition duration-300 transform hover:scale-105"
              >
                <img
                  src={match.image_url}
                  alt={match.name}
                  className="w-full h-48 object-cover rounded mb-2"
                />
                <div>
                  <h2 className="text-xl font-semibold">
                    {match.name}, {match.age}
                  </h2>
                  <p className="text-gray-600">{match.bio}</p>
                </div>
              </div>
            ))
          ) : (
            <div className="bg-white rounded shadow-md p-4">
              <h2 className="text-xl font-semibold">
                <p className="text-black">
                  No matches yet. Swipe to find some!
                </p>
              </h2>
            </div>
          )}
        </div>
      </div>
    </>
  );
}
