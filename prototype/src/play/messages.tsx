import { useEffect, useRef, useState } from "preact/hooks"
import { receive } from "../server"
import type { RefObject } from "preact"

export function Messages() {
  const [data, setData] = useState<{ title: string, content: string }[]>([])
  const ref = useRef<HTMLDivElement>(null)
  useEffect(() => {
    return receive<{ title: string, content: string }>("message", (message) => {
      setData((data) => [...data, message ])
    })
  }, [])
  return (
    <div style={{ flex: 1, overflow: "auto", borderBottom: "3px solid white" }} ref={ref}>
      {data.map((message, index) => (
        <div key={index} style={{ border: "1px solid white", padding: 10, margin: 10 }}>
          <strong><TypeWriter text={message.title} scroll={ref}/></strong>
          <p style={{ wordBreak: "break-word" }}>
            <TypeWriter text={message.content} scroll={ref}/>
          </p>
        </div>
      ))}
    </div>
  )
}

function TypeWriter(props: { text: string, scroll?: RefObject<HTMLDivElement> }) {
  const [text, setText] = useState("")
  useEffect(() => {
    if(text.length >= props.text.length) return
    let index = 0
    const interval = setInterval(() => {
      setText((text) => text + props.text[index])
      props.scroll?.current?.scrollTo(0, props.scroll.current.scrollHeight)
      index++
      if(index === props.text.length) clearInterval(interval)
    }, 1000 / props.text.length)
    return () => clearInterval(interval)
  }, [])
  return <>{text}</>
}
