import { useEffect, useRef } from "react";
export default function useWebsocket(onMessage: any, room_id: string) {
  const ws = useRef<WebSocket | null>(null);

  useEffect(() => {
    if (ws.current || !room_id) return;
    const url = process.env.NEXT_PUBLIC_BASE_URL?.slice(7); // get rid of https://
    const wsUri = `ws://${url}/chat/${room_id}`;
    ws.current = new WebSocket(wsUri);
    ws.current.onopen = () => console.log("ws opened");
    ws.current.onclose = () => console.log("ws closed");
    ws.current.onerror = (error) => console.log("ws error: ", error);
    const wsCurrent = ws.current;
    return () => {
      if (wsCurrent.readyState === WebSocket.OPEN) wsCurrent.close();
    };
  }, []);
  useEffect(() => {
    if (!ws.current) return;
    ws.current.onmessage = (e) => {
      onMessage(e.data);
    };
  }, []);
  const sendMessage = (msg: any) => {
    if (!ws.current) return;
    ws.current.send(msg);
  };
  return sendMessage;
}
