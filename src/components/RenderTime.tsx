import { useEffect, useState } from "react";

interface Props {
  createdAt: string;
}

function RenderTime({ createdAt }: Props) {
  const [currentTime, setCurrentTime] = useState(new Date());

  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentTime(new Date());
    }, 10000);

    return () => {
      clearInterval(timer);
    };
  }, []);

  const ago = Math.floor(
    (currentTime.getTime() - new Date(createdAt).getTime()) / 1000
  );

  const time =
    ago >= 31536000
      ? `${Math.floor(ago / 31536000)}年前`
      : ago >= 2592000
      ? `${Math.floor(ago / 2592000)}か月前`
      : ago >= 604800
      ? `${Math.floor(ago / 604800)}週間前`
      : ago >= 86400
      ? `${Math.floor(ago / 86400)}日前`
      : ago >= 3600
      ? `${Math.floor(ago / 3600)}時間前`
      : ago >= 60
      ? `${Math.floor(ago / 60)}分前`
      : ago >= 10
      ? `${Math.floor(ago)}秒前`
      : ago >= -1
      ? "たった今"
      : "未来";
  return <>{time}</>;
}

export default RenderTime;
