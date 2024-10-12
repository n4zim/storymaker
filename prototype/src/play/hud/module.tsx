import { ReactNode } from "preact/compat"

export function Module(props: { children: ReactNode }) {
  return (
    <div style={{ border: "1px solid white", padding: 10, flex: "1 1 auto" }}>
      {props.children}
    </div>
  )
}
