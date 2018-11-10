import { Component, OnInit } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import { Contact, ContactType } from "../contact-model.service";
import { session } from "../model";
import { HttpClient, HttpHeaders } from "@angular/common/http";

@Component({
  selector: "contact-create-page",
  templateUrl: "./contact-create-page.component.html",
  styleUrls: ["./contact-create-page.component.css"]
})
export class ContactCreatePageComponent implements OnInit {
  constructor(
    private http: HttpClient,
    private route: ActivatedRoute,
    private router: Router
  ) {}

  queryTypes() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    this.http
      .get("/api/v1/contacts/types", options)
      .subscribe((response: any) => {
        console.log(response);
      });
  }

  ngOnInit() {
    this.queryTypes();
  }

  created() {
    this.router.navigate(["../"], { relativeTo: this.route });
  }
}
