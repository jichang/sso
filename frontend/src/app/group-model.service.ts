import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { HttpHeaders, HttpClient } from "@angular/common/http";

export interface Group {
  id: number;
  name: string;
  status: number;
}

export interface GroupStore {
  groups: Group[];
}

@Injectable()
export class GroupModelService {
  private store: GroupStore;
  private subject: BehaviorSubject<Group[]>;

  constructor(private http: HttpClient) {
    this.store = {
      groups: []
    };
    this.subject = new BehaviorSubject<Group[]>([]);
  }

  get groups() {
    return this.subject.asObservable();
  }

  select() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/groups`;
    return this.http.get(apiUri, options).subscribe(
      (groups: Group[]) => {
        this.store.groups = groups;
        this.subject.next(groups);
      },
      err => {
        this.subject.error(err);
      }
    );
  }
}
