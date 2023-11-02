import React from "react";

export default function ProfileComponent({
  text,
  func,
  required,
  placeholder,
  value,
}: {
  text: string;
  placeholder: string;
  func: any;
  required: boolean;
  value: string;
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
        value={value}
        placeholder={placeholder}
        onChange={(e) => func(e.target.value)}
        className="w-full text-black px-3 py-2 border rounded focus:outline-none focus:border-blue-400"
        required={required}
      />
    </div>
  );
}
