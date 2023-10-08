"use client";

import styles from '../page.module.css';
import { useState, useEffect } from "react";
import { Person, Page } from "@/bindings";
import { personController } from "@/api/api";

interface Props {
    reRender: boolean
}

export const ListPerson = ({ reRender }: Props) => {
    const ITEMS_PER_PAGE = 10;

    const [personList, setPersonList] = useState<Person[]>([]);
    const [removed, setRemoved] = useState<boolean>(false); //effectful state
    const [page, setPage] = useState<Page>({ page: 0, limit: ITEMS_PER_PAGE });

    useEffect(() => {
        const listPerson = async () => {
            personController.list(page).then(
                (result) => { setPersonList(result) },
                (reason) => { console.error("list call rejected: ", reason) },
            )
        }
        listPerson();
    }, [reRender, removed, page]);

    useEffect(() => {
        if ((personList.length == 0) && (page.page > 0)) {
            setPage({ ...page, page: page.page - 1 })
        }
    }, [personList, page])

    const deletePerson = async (id: string) => {
        try {
            let person_id = await personController.delete(id);
            setRemoved(!removed);
        } catch (e) {
            console.error("error = ", e)
        }
    }

    return (
        <div className={styles.container}>
            <div className={styles.pagination}>
                <button className={styles.pageButton} onClick={() => {
                    if (page.page > 0) {
                        setPage({ ...page, page: page.page - 1 })
                    }
                }}>&laquo;</button>
                <button className={styles.pageButton} style={{ pointerEvents: "none", backgroundColor: "white" }} disabled>
                    {page.page + 1}
                </button>
                <button className={styles.pageButton} onClick={() => {
                    if (personList.length >= ITEMS_PER_PAGE) {
                        setPage({ ...page, page: page.page + 1 })
                    }
                }}>&raquo;</button>
            </div>
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
