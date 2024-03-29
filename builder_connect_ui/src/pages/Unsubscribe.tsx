import React, { useState } from "react";
import { useLocalStorage } from "usehooks-ts";

export default function Unsubscribe() {
  const [sub_id, set_sub_id] = useLocalStorage("sub_id", "");
  const [unsubscribed, set_unsubscribed] = useState(false);
  async function unsubscribe() {
    try {
      if (sub_id === "") return;
      const url = process.env.NEXT_PUBLIC_BASE_URL + `/mailing_list/${sub_id}`;
      const res = await fetch(url, {
        method: "DELETE",
        credentials: "include",
      });
      const data = await res.text();
      console.log(data);
      if (data === "Deleted from mailing list") {
        set_sub_id("");
        set_unsubscribed(true);
      }
    } catch (err) {
      console.log(err);
    }
  }
  return (
    <div>
      {unsubscribed ? (
        <h1>You have successfully unsubscribed</h1>
      ) : (
        <button onClick={unsubscribe}>Unsubscribe</button>
      )}
    </div>
  );
}
