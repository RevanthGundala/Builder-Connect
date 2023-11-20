import Navbar from "@/components/Navbar";
import ParticleBackground from "@/components/ParticleBackground";
import { NextRouter, useRouter } from "next/router";
import React, { useEffect, useState } from "react";
import { useLocalStorage } from "usehooks-ts";

export default function Matches() {
  const router = useRouter();
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");
  const [matches, set_matches] = useState<any>([]);

  useEffect(() => {
    view_matches();

    async function view_matches() {
      if (sub_id === "") return;
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/matches/${sub_id}`;
      const res = await fetch(url);
      const new_match_ids = await res.json();
      const new_matches = await Promise.all(
        new_match_ids.map(async (id: number) => {
          const url = process.env.NEXT_PUBLIC_BASE_URL + `/view/${id}`;
          const res = await fetch(url);
          const data = await res.json();
          return data;
        })
      );
      set_matches(new_matches);
    }
  }, [matches, sub_id]);

  return (
    <>
      <ParticleBackground />
      <Navbar />
      <div className="pt-16 bg-cover bg-center relative mx-auto">
        <h1 className="text-2xl text-white font-semibold mb-4">Your Matches</h1>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {sub_id !== "" && matches.length > 0 ? (
            matches.map((match: any) => (
              <div
                key={match.sub_id}
                className="bg-white rounded shadow-md p-4 cursor-pointer transition duration-300 transform hover:scale-105"
              >
                <img
                  src={match.image_url}
                  alt={match.username}
                  className="w-full h-48 object-cover rounded mb-2"
                />
                <div>
                  <h2 className="text-xl font-semibold">
                    {match.username}, {match.age}
                  </h2>
                  <p className="text-gray-600">{match.reason}</p>
                  <p className="text-gray-600">{match.project_interests}</p>
                </div>
              </div>
            ))
          ) : sub_id !== "" && matches.length === 0 ? (
            <div className="bg-white rounded shadow-md p-4">
              <h2 className="text-xl font-semibold">
                <p className="text-white">
                  No matches yet. Swipe to find some!
                </p>
              </h2>
            </div>
          ) : (
            <div>Loading...</div>
          )}
        </div>
      </div>
    </>
  );
}
