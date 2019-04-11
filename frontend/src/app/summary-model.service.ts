import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import { session } from "./model";

export interface SummaryQuota {
  enabled: boolean;
  total: number;
  used: number;
}

export interface Summary {
  users: SummaryQuota;
  roles: SummaryQuota;
  groups: SummaryQuota;
  applications: SummaryQuota;
  authorizations: SummaryQuota;
  contacts: SummaryQuota;
  invitations: SummaryQuota;
}

export interface SummaryStore {
  summary?: Summary;
}

@Injectable({
  providedIn: "root"
})
export class SummaryModelService {
  private store: SummaryStore = {
    summary: null
  };
  private subject: BehaviorSubject<Summary> = new BehaviorSubject(null);

  constructor(private http: HttpClient) {}

  get summary() {
    return this.subject.asObservable();
  }

  select() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = "/api/v1/users/" + session.currUser().id + "/summary";
    this.http.get(apiUri, options).subscribe((summary: Summary) => {
      this.store.summary = summary;
      this.subject.next(summary);
    });
  }
}
