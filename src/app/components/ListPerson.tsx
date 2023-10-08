"use client";

import styles from '../page.module.css';
import { useState, useEffect } from "react";
import { Person } from "@/bindings";
import { personController } from "@/api/api";

interface Props {
    reRender: boolean
}

export const ListPerson = ({ reRender }: Props) => {
    const [personList, setPersonList] = useState<Person[]>([]);
    const [removed, setRemoved] = useState<boolean>(false); //effectful state

    useEffect(() => {
        const listPerson = async () => {
            personController.list().then(
                (result) => { setPersonList(result) },
                (reason) => { console.error("list call rejected: ", reason) },
            )
        }
        listPerson();
    }, [reRender, removed]);

    const deletePerson = async (id: string) => {
        let splitted = id.split(":");
        try {
            let person_id = await personController.delete(splitted[1]);
            setRemoved(!removed);
        } catch (e) {
            console.error("error = ", e)
        }
    }

    return (
        <div className={styles.container}>
            <table className={styles.personListTable}>
                <thead>
                    <tr>
                        <th>Title</th>
                        <th>First Name</th>
                        <th>Last Name</th>
                        <th>Marketing</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {personList?.map((person, i) => {
                        return [
                            <tr key={i}>
                                <td>{person.title}</td>
                                <td>{person.name.first}</td>
                                <td>{person.name.last}</td>
                                <td>{person.marketing ? "X" : "O"}</td>
                                <td>
                                    <button className={styles.deleteButton} onClick={() => {
                                        deletePerson(person.id);
                                    }}>X</button>
                                </td>
                            </tr>
                        ];
                    })}
                </tbody>
            </table>
        </div >
    )
}
