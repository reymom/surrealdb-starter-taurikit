"use client";

import styles from './page.module.css'
import { useState } from "react"
import dynamic from 'next/dynamic'

//we need to import dynamically the components that call the tauri invoke as a workaround
//to avoid the `no window found` error due to nextjs running a Node.js server for SSR
const ListPerson = dynamic(
    () => import("@/app/components/ListPerson").then(
        (mod) => mod.ListPerson
    ), { ssr: false }
)
const AddPerson = dynamic(
    () => import("@/app/components/AddPerson").then(
        (mod) => mod.AddPerson
    ), { ssr: false }
)


export default function Home() {
    //effectful state
    const [reRender, setRerender] = useState<boolean>(false);

    /* we want to rerender the list when a new person is added
     reRender is changed by onCreate triggered in AddPerson 
     it will be passed to and used as an effect in ListPerson */
    const onCreate = (() => {
        setRerender(!reRender)
    });

    return (
        <main className={styles.main}>
            <AddPerson onCreate={onCreate} />
            <ListPerson reRender={reRender} />
        </main>
    )
}
