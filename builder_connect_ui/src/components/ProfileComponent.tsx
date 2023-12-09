import React from "react";

export default function ProfileComponent({
  text,
  func,
  required,
  placeholder,
  value,
  descriptive,
}: {
  text: string;
  placeholder: string;
  func: any;
  required: boolean;
  value: string;
  descriptive: boolean;
}) {
  return (
    <div className="mb-4">
      <label htmlFor="name" className="text-gray-600 flex">
        {text} <p className="text-red-500 pl-1">{required ? "*" : ""}</p>
      </label>
      {!descriptive ? (
        <input
          type="text"
          defaultValue={value}
          placeholder={placeholder}
          onChange={(e) => func(e.target.value)}
          className="w-full text-gray-900 px-3 py-2 border rounded focus:outline-none focus:border-blue-400"
          required={required}
          maxLength={100}
        />
      ) : (
        <textarea
          defaultValue={value}
          placeholder={placeholder}
          onChange={(e) => func(e.target.value)}
          className="block px-3 py-2 w-full text-sm text-gray-900 border rounded focus:border-blue-400 rows-4"
          required={required}
        />
      )}
    </div>
  );
}
