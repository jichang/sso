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
import { TokenModelService, CreateParams } from "../token-model.service";

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
  @ViewChild(MatVerticalStepper, { static: true })
  private stepper: MatVerticalStepper;
  private contact?: Contact;

  constructor(
    private http: HttpClient,
    private route: ActivatedRoute,
    private router: Router,
    private snackBar: MatSnackBar,
    private tokenModel: TokenModelService
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

    let queryParams = this.route.snapshot.queryParams;
    if (queryParams.target_id && queryParams.target_type && queryParams.token) {
      this.steps[0].completed = true;
      this.steps[0].editable = false;
      this.steps[1].completed = true;
      this.steps[1].editable = false;
      this.stepper.selectedIndex = 2;

      this.tokenModel
        .remove({
          target_id: queryParams.target_id,
          target_type: queryParams.target_type,
          action: "verify",
          token: queryParams.token
        })
        .subscribe(response => {
          this.snackBar.open("contact verified", "Dismiss", {
            duration: 3000
          });
        });
    }
  }

  created(contact: Contact) {
    this.contact = contact;
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

  createToken() {
    let params: CreateParams = {
      target_id: this.contact.id,
      target_type: "email",
      target_identity: this.contact.identity,
      action: "verify"
    };

    this.tokenModel.create(params).subscribe(() => {
      this.snackBar.open("Verify email send", "Dismiss", {
        duration: 3000
      });

      this.stepper.next();
    });
  }
}
