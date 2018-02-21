import { Component, OnInit } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { Router } from "@angular/router";

@Component({
  selector: "dashboard-nav",
  templateUrl: "./dashboard-nav.component.html",
  styleUrls: ["./dashboard-nav.component.css"]
})
export class DashboardNavComponent implements OnInit {
  constructor(private http: HttpClient, private router: Router) {}

  ngOnInit() {}

  signOut() {
    this.http.post("/api/v1/signout", {}).subscribe(
      (response: Response) => {
        window.localStorage.removeItem("jwt");
        window.localStorage.removeItem("user");

        this.router.navigate(["/"]);
      },
      (response: Response) => {}
    );
  }
}
