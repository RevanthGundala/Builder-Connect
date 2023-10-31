import React from "react";

export default function ProfileComponent({
  text,
  func,
  required,
}: {
  text: string;
  func: any;
  required: boolean;
}) {
  return (
    <div className="mb-4">
      <label htmlFor="name" className="block text-gray-600 flex">
        {text} <p className="text-red-500 pl-1">{required ? "*" : ""}</p>
      </label>
      <input
        id="name"
        name="name"
        type="text"
        onChange={(e) => func(e.target.value)}
        className="w-full text-black px-3 py-2 border rounded focus:outline-none focus:border-blue-400"
        required={required}
      />
    </div>
  );
}
