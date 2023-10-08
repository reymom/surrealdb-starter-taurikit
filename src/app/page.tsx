"use client";

import styles from './page.module.css'
import { ListPerson, AddPerson } from "@/app/components"
import { useState } from "react"

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
