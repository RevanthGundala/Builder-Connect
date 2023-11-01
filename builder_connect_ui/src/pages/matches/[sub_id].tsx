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
  const [matches, set_matches] = useState<any>([]);

  useEffect(() => {
    view_matches(router);

    async function view_matches(router: NextRouter) {
      const sub_id = router.asPath.split("/")[2];
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
  }, [matches]);

  return (
    <div className="bg-gray-100 p-4">
      <h1 className="text-2xl text-black font-semibold mb-4">Your Matches</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {matches.map((match: any) => (
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
        ))}
      </div>
    </div>
  );
}
