import { useCallback, useEffect, useState } from "react";
const fetch_room_data = async (room_id: string) => {
  if (!room_id) return;
  const url = `${process.env.NEXT_PUBLIC_BASE_URL}/messages/${room_id}`;
  try {
    let resp = await fetch(url).then((res) => res.json());
    return resp;
  } catch (e) {
    console.log(e);
  }
};

type ConversationsHookReturnType = [
  boolean,
  any[],
  React.Dispatch<React.SetStateAction<any[]>>,
  (id: string) => void
];

export default function useConversations(
  room_id: string
): ConversationsHookReturnType {
  const [is_loading, set_is_loading] = useState(true);
  const [messages, set_messages] = useState<any[]>([]);
  const update_messages = (resp = []) => {
    set_is_loading(false);
    set_messages(resp);
    console.log("room_id", room_id);
    console.log("resp", resp);
    console.log("msgs: ", messages);
  };
  const fetch_conversations = (id: string) => {
    set_is_loading(true);
    fetch_room_data(id).then(update_messages);
  };
  // useEffect(() => fetch_conversations(room_id), []);
  return [is_loading, messages, set_messages, fetch_conversations];
}
