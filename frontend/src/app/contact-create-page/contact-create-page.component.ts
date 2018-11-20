import { Component, OnInit, ViewChild } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import { Contact, ContactType } from "../contact-model.service";
import { session } from "../model";
import {
  HttpClient,
  HttpHeaders,
  HttpErrorResponse
} from "@angular/common/http";
import { MatSnackBar, MatVerticalStepper } from "@angular/material";

interface Step {
  completed: boolean;
  editable: boolean;
}

@Component({
  selector: "contact-create-page",
  templateUrl: "./contact-create-page.component.html",
  styleUrls: ["./contact-create-page.component.css"]
})
export class ContactCreatePageComponent implements OnInit {
  steps: Step[] = [
    {
      completed: false,
      editable: false
    },
    {
      completed: false,
      editable: false
    },
    {
      completed: false,
      editable: false
    }
  ];
  @ViewChild(MatVerticalStepper)
  private stepper: MatVerticalStepper;

  constructor(
    private http: HttpClient,
    private route: ActivatedRoute,
    private router: Router,
    private snackBar: MatSnackBar
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
    this.snackBar.open("Email created", "Dismiss", {
      duration: 3000
    });

    this.stepper.next();
  }

  failure(err: HttpErrorResponse) {
    if (err.status === 409) {
      this.snackBar.open("Email used", "Dismiss", {
        duration: 3000
      });
    } else {
      this.snackBar.open("Unknown error", "Dismiss", {
        duration: 3000
      });
    }
  }
}
