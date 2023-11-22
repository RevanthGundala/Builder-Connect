import React from "react";
import { EnvelopeIcon } from "@heroicons/react/24/solid";
export default function Sidebar({
  matches,
  sub_id,
  set_sub_id,
}: {
  matches: any[] | undefined;
  sub_id: string;
  set_sub_id: React.Dispatch<React.SetStateAction<string>>;
}) {
  return (
    <div>
      {matches && matches.length > 0 ? (
        matches.map((match: any, index: number) => (
          <div
            key={index}
            className="flex flex-col text-black"
            onClick={() => set_sub_id(match.sub_id)}
          >
            {match.sub_id === sub_id ? (
              <div className="flex flex-row space-x-2 bg-gray-300 py-4 px-2">
                <img
                  src={match.image_url}
                  alt={match.username}
                  className="w-6 h-6 object-cover rounded-full"
                />
                <div className="flex flex-col">
                  <p>{match.username}</p>
                  <p>Placeholder for message</p>
                </div>
              </div>
            ) : (
              <div className="flex flex-row space-x-2 bg-gray-300 opacity-70 py-4 px-2">
                <img
                  src={match.image_url}
                  alt={match.username}
                  className="w-6 h-6 object-cover rounded-full"
                />
                <div className="flex flex-col">
                  <p>{match.username}</p>
                  <p>Placeholder for message</p>
                </div>
              </div>
            )}
          </div>
        ))
      ) : (
        <div className="pt-4 bg-cover bg-center">
          <div className="flex flex-col space-y-2">
            <EnvelopeIcon className="w-16 h-16 text-white mx-auto" />
            <p className="text-white text-xl text-center">No matches yet!</p>
          </div>
        </div>
      )}
    </div>
  );
}
