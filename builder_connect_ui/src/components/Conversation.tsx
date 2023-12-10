import React, { useEffect, useRef } from "react";
function ConversationItem({
  right,
  content,
  created_at,
}: {
  right: boolean;
  content: string;
  created_at: string;
}) {
  if (right) {
    return (
      <div className="w-full flex justify-end">
        <div className="flex flex-col space-y-1 items-end">
          <p className="text-sm text-gray-600">{created_at}</p>
          <div className="flex gap-3 justify-end">
            <div className="max-w-fit bg-blue-500 p-3 text-sm rounded-xl rounded-br-none hs">
              <p className="text-white text-md">{content}</p>
            </div>
          </div>
        </div>
      </div>
    );
  } else {
    return (
      <div className="w-full flex justify-start">
        <div className="flex flex-col space-y-1 items-start">
          <p className="text-sm text-gray-600">{created_at}</p>
          <div className="flex gap-3 justify-start">
            <div className="max-w-fit bg-gray-600 p-3 text-sm rounded-xl rounded-bl-none">
              <p className="text-white text-md">{content}</p>
            </div>
          </div>
        </div>
      </div>
    );
  }
}
export default function Conversation({
  messages,
  sub_id,
}: {
  messages: any[];
  sub_id: string;
}) {
  return (
    <div className="p-4 space-y-4">
      {messages &&
        messages.length > 0 &&
        messages.map((message: any, index: number) => {
          return (
            <ConversationItem
              key={index}
              right={message.user_sub_id === sub_id}
              content={message.content}
              created_at={
                message.should_display
                  ? new Date(message.created_at).toLocaleString(undefined, {
                      weekday: "short",
                      year: "numeric",
                      month: "short",
                      day: "numeric",
                      hour: "numeric",
                      minute: "numeric",
                    })
                  : ""
              }
            />
          );
        })}
    </div>
  );
}
